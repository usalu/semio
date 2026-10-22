"""🧭️ Independent Python model of the CI validation resolution (standard library only): from a GitHub environment and event it derives the destination branch, reads the paged workflow history through the recorded responses, selects the newest successful trusted ancestor and counts the requests and ancestry questions it needed. Requests arrive as JSON on stdin."""
import json, re, sys

SHA = re.compile(r"[a-f0-9]{40}")
REPO = re.compile(r"(?!\.\.?/)[A-Za-z0-9_.-]+/(?!\.\.?$)[A-Za-z0-9_.-]+")


class Refusal(Exception):
    pass


def ident(value):
    return isinstance(value, int) and not isinstance(value, bool) and 0 < value <= 2 ** 53 - 1


def text(value):
    return isinstance(value, str) and value != "" and not re.search(r"[\x00-\x1f]", value)


def env_identity(value):
    if not value or not re.fullmatch(r"[1-9][0-9]*", value) or int(value) > 2 ** 53 - 1:
        raise Refusal("Invalid GitHub identity")
    return int(value)


def obj(value, message):
    if not isinstance(value, dict):
        raise Refusal(message)
    return value


def environment(env, event, head, full):
    if not SHA.fullmatch(head) or not isinstance(full, bool):
        raise Refusal("Invalid CI checkout")
    if env.get("GITHUB_ACTIONS") != "true":
        return {"history": None}
    name = env.get("GITHUB_EVENT_NAME", "")
    if name not in ("push", "pull_request", "schedule", "workflow_dispatch"):
        raise Refusal("Unsupported GitHub validation event")
    repository = env.get("GITHUB_REPOSITORY")
    repository_id, run_id = env_identity(env.get("GITHUB_REPOSITORY_ID")), env_identity(env.get("GITHUB_RUN_ID"))
    payload = obj(event, "Invalid GitHub event")
    source = obj(payload.get("repository"), "Invalid GitHub event")
    if not repository or not REPO.fullmatch(repository) or env.get("GITHUB_SHA") != head or source.get("id") != repository_id or not isinstance(source.get("full_name"), str) or source["full_name"].lower() != repository.lower():
        raise Refusal("Invalid GitHub checkout or repository identity")
    branch = None
    if name == "pull_request":
        base = obj(obj(payload.get("pull_request"), "Invalid GitHub event").get("base"), "Invalid GitHub event")
        if obj(base.get("repo"), "Invalid GitHub event").get("id") != repository_id:
            raise Refusal("Invalid pull request destination repository")
        branch = base.get("ref")
        if not text(branch):
            raise Refusal("Invalid pull request destination branch")
    elif env.get("GITHUB_REF", "").startswith("refs/heads/"):
        branch = env["GITHUB_REF"][len("refs/heads/"):]
        if not text(branch):
            raise Refusal("Invalid GitHub branch")
    if full or name == "schedule" or not branch:
        return {"history": None}
    return {"history": {"repository": repository, "repositoryId": repository_id, "runId": run_id, "branch": branch}}


def history(request, scenario, counter):
    pages = 0
    try:
        counter["requests"] += 1
        answer = scenario["run"]
        if answer["status"] != 200:
            raise Refusal("x")
        current = obj(answer["body"], "x")
        if current.get("id") != request["runId"] or not ident(current.get("workflow_id")) or obj(current.get("repository"), "x").get("id") != request["repositoryId"]:
            raise Refusal("x")
        workflow, runs = current["workflow_id"], []
        for page in range(1, 11):
            counter["requests"] += 1
            index = counter["requests"] - 2
            answer = scenario["pages"][index] if index < len(scenario["pages"]) else {"status": 404, "body": None}
            if answer["status"] != 200:
                raise Refusal("x")
            body = obj(answer["body"], "x")
            total, items = body.get("total_count"), body.get("workflow_runs")
            if not ident(total) and total != 0 or not isinstance(items, list) or len(items) > 100:
                raise Refusal("x")
            for value in items:
                run = obj(value, "x")
                if run.get("head_repository") is None or run.get("head_branch") is None:
                    continue
                repo = obj(run["head_repository"], "x")
                if not all(ident(v) for v in (run.get("id"), run.get("run_number"), run.get("workflow_id"), repo.get("id"))) or not all(text(v) for v in (run["head_branch"], run.get("event"), run.get("status"))) or not isinstance(run.get("head_sha"), str) or not SHA.fullmatch(run["head_sha"]) or (run.get("conclusion") is not None and not text(run.get("conclusion"))):
                    raise Refusal("x")
                runs.append({"id": run["id"], "number": run["run_number"], "repositoryId": repo["id"], "workflowId": run["workflow_id"], "branch": run["head_branch"], "head": run["head_sha"], "event": run["event"], "status": run["status"], "conclusion": run.get("conclusion")})
            pages += 1
            if len(items) < 100 or page * 100 >= total:
                return {"workflowId": workflow, "runs": runs, "issue": None}
        return {"workflowId": workflow, "runs": runs, "issue": "history-limit"}
    except Refusal:
        return {"workflowId": None, "runs": [], "issue": "history-unavailable"}


def resolve(scenario):
    counter = {"requests": 0, "ancestors": 0}
    try:
        selected = environment(scenario["environment"], scenario["event"], scenario["head"], scenario["full"])
    except Refusal as error:
        return {"error": str(error)}
    head = scenario["head"]
    if selected["history"] is None:
        return {"baseline": {"mode": "all", "base": None, "runId": None, "reason": "full-requested"}, "issue": None, "requests": 0, "ancestorCalls": 0}
    found = history(selected["history"], scenario, counter)
    if found["workflowId"] is None:
        return {"baseline": {"mode": "all", "base": None, "runId": None, "reason": "no-successful-ancestor"}, "issue": found["issue"], "requests": counter["requests"], "ancestorCalls": 0}
    context = selected["history"]
    candidates = [r for r in found["runs"] if r["id"] != context["runId"] and r["repositoryId"] == context["repositoryId"] and r["workflowId"] == found["workflowId"] and r["branch"] == context["branch"] and r["status"] == "completed" and r["conclusion"] == "success" and r["event"] in ("push", "schedule", "workflow_dispatch")]
    candidates.sort(key=lambda r: (-r["number"], -r["id"]))
    for run in candidates:
        counter["ancestors"] += 1
        if scenario["ancestors"].get(run["head"], False):
            return {"baseline": {"mode": "affected", "base": run["head"], "runId": run["id"], "reason": "last-successful-ancestor"}, "issue": found["issue"], "requests": counter["requests"], "ancestorCalls": counter["ancestors"]}
    return {"baseline": {"mode": "all", "base": None, "runId": None, "reason": "no-successful-ancestor"}, "issue": found["issue"], "requests": counter["requests"], "ancestorCalls": counter["ancestors"]}


print(json.dumps([resolve(s) for s in json.load(sys.stdin)["scenarios"]]))
