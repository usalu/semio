"""📝 Authoring aid: writes the CI resolution scenario corpus (corpus.json); expectations of the key scenarios are authored here, the rest is read from an independent model at test time."""
import json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"
HEAD = "a" * 40
B1, B2, B3 = "b" * 40, "c" * 40, "d" * 40
REPO, REPO_ID, RUN_ID, WORKFLOW = "semio/repo", 11, 99, 7


def env(**k):
    base = {"GITHUB_ACTIONS": "true", "GITHUB_EVENT_NAME": "push", "GITHUB_REPOSITORY": REPO, "GITHUB_REPOSITORY_ID": str(REPO_ID), "GITHUB_RUN_ID": str(RUN_ID), "GITHUB_SHA": HEAD, "GITHUB_REF": "refs/heads/main"}
    base.update(k)
    return {key: value for key, value in base.items() if value is not None}


def event(**k):
    base = {"repository": {"id": REPO_ID, "full_name": "Semio/Repo"}, "pull_request": {"base": {"ref": "main", "repo": {"id": REPO_ID}}}}
    base.update(k)
    return base


def run_body(**k):
    base = {"id": RUN_ID, "workflow_id": WORKFLOW, "repository": {"id": REPO_ID}}
    base.update(k)
    return base


def wf(id, number, head, **k):
    base = {"id": id, "run_number": number, "workflow_id": WORKFLOW, "head_repository": {"id": REPO_ID}, "head_branch": "main", "event": "push", "status": "completed", "conclusion": "success", "head_sha": head}
    base.update(k)
    return base


def page(runs, total=None, status=200):
    return {"status": status, "body": {"total_count": len(runs) if total is None else total, "workflow_runs": runs}}


def sc(name, e=None, ev="default", head=HEAD, full=False, run=None, pages=None, ancestors=None, expect=None):
    return {"name": name, "environment": e if e is not None else env(), "event": event() if ev == "default" else ev, "head": head, "full": full,
            "run": run if run is not None else {"status": 200, "body": run_body()}, "pages": pages if pages is not None else [page([])], "ancestors": ancestors or {}, "expect": expect}


def ok(mode, base=None, run=None, reason="", issue=None, requests=0, ancestors=0):
    return {"mode": mode, "base": base, "runId": run, "reason": reason, "issue": issue, "error": "", "requests": requests, "ancestorCalls": ancestors}


def fail(error):
    return {"mode": None, "base": None, "runId": None, "reason": "", "issue": None, "error": error, "requests": None, "ancestorCalls": None}


scenarios = [
    sc("the nearest successful ancestor is the base", pages=[page([wf(1, 10, B1), wf(2, 12, B2)])], ancestors={B1: True, B2: True}, expect=ok("affected", B2, 2, "last-successful-ancestor", None, 2, 1)),
    sc("a newer run that is not an ancestor is skipped", pages=[page([wf(1, 10, B1), wf(2, 12, B2)])], ancestors={B1: True, B2: False}, expect=ok("affected", B1, 1, "last-successful-ancestor", None, 2, 2)),
    sc("no ancestor means full validation", pages=[page([wf(1, 10, B1)])], ancestors={B1: False}, expect=ok("all", None, None, "no-successful-ancestor", None, 2, 1)),
    sc("an empty history means full validation", pages=[page([])], expect=ok("all", None, None, "no-successful-ancestor", None, 2, 0)),
    sc("the run being validated is never its own base", pages=[page([wf(RUN_ID, 20, B1), wf(2, 5, B2)])], ancestors={B1: True, B2: True}, expect=ok("affected", B2, 2, "last-successful-ancestor", None, 2, 1)),
    sc("equal run numbers are ordered by id", pages=[page([wf(3, 10, B1), wf(5, 10, B2)])], ancestors={B1: True, B2: True}, expect=ok("affected", B2, 5, "last-successful-ancestor", None, 2, 1)),
    sc("a run of another branch is not a base", pages=[page([wf(1, 10, B1, head_branch="dev")])], ancestors={B1: True}, expect=ok("all", None, None, "no-successful-ancestor", None, 2, 0)),
    sc("a pull request run is not a base", pages=[page([wf(1, 10, B1, event="pull_request")])], ancestors={B1: True}, expect=ok("all", None, None, "no-successful-ancestor", None, 2, 0)),
    sc("scheduled and dispatched runs are bases", pages=[page([wf(1, 10, B1, event="schedule"), wf(2, 11, B2, event="workflow_dispatch")])], ancestors={B1: True, B2: True}, expect=ok("affected", B2, 2, "last-successful-ancestor", None, 2, 1)),
    sc("a failed run is not a base", pages=[page([wf(1, 10, B1, conclusion="failure")])], ancestors={B1: True}, expect=ok("all", None, None, "no-successful-ancestor", None, 2, 0)),
    sc("an unfinished run is not a base", pages=[page([wf(1, 10, B1, status="in_progress", conclusion=None)])], ancestors={B1: True}, expect=ok("all", None, None, "no-successful-ancestor", None, 2, 0)),
    sc("a run of a fork is not a base", pages=[page([wf(1, 10, B1, head_repository={"id": 12})])], ancestors={B1: True}, expect=ok("all", None, None, "no-successful-ancestor", None, 2, 0)),
    sc("a run of another workflow is not a base", pages=[page([wf(1, 10, B1, workflow_id=8)])], ancestors={B1: True}, expect=ok("all", None, None, "no-successful-ancestor", None, 2, 0)),
    sc("a run whose repository was deleted is dropped", pages=[page([wf(1, 10, B1, head_repository=None), wf(2, 9, B2)])], ancestors={B1: True, B2: True}, expect=ok("affected", B2, 2, "last-successful-ancestor", None, 2, 1)),
    sc("a run without a branch is dropped", pages=[page([wf(1, 10, B1, head_branch=None), wf(2, 9, B2)])], ancestors={B1: True, B2: True}, expect=ok("affected", B2, 2, "last-successful-ancestor", None, 2, 1)),
    sc("a malformed run makes the whole history unavailable", pages=[page([wf(1, 10, "xyz")])], expect=ok("all", None, None, "no-successful-ancestor", "history-unavailable", 2, 0)),
    sc("an explicit full request skips the network", full=True, expect=ok("all", None, None, "full-requested", None, 0, 0)),
    sc("outside GitHub Actions the whole graph is validated", e={}, expect=ok("all", None, None, "full-requested", None, 0, 0)),
    sc("a scheduled run validates everything", e=env(GITHUB_EVENT_NAME="schedule"), expect=ok("all", None, None, "full-requested", None, 0, 0)),
    sc("a tag build validates everything", e=env(GITHUB_REF="refs/tags/v1"), expect=ok("all", None, None, "full-requested", None, 0, 0)),
    sc("a build without a ref validates everything", e=env(GITHUB_REF=None), expect=ok("all", None, None, "full-requested", None, 0, 0)),
    sc("a pull request uses its destination branch", e=env(GITHUB_EVENT_NAME="pull_request", GITHUB_REF="refs/pull/1/merge"), ev=event(pull_request={"base": {"ref": "dev", "repo": {"id": REPO_ID}}}), pages=[page([wf(1, 10, B1, head_branch="dev")])], ancestors={B1: True}, expect=ok("affected", B1, 1, "last-successful-ancestor", None, 2, 1)),
    sc("a pull request into a foreign repository is refused", e=env(GITHUB_EVENT_NAME="pull_request"), ev=event(pull_request={"base": {"ref": "dev", "repo": {"id": 99}}}), expect=fail("Invalid pull request destination repository")),
    sc("a pull request without a destination branch is refused", e=env(GITHUB_EVENT_NAME="pull_request"), ev=event(pull_request={"base": {"repo": {"id": REPO_ID}}}), expect=fail("Invalid pull request destination branch")),
    sc("an unsupported event is refused", e=env(GITHUB_EVENT_NAME="release"), expect=fail("Unsupported GitHub validation event")),
    sc("a checkout that is not the event commit is refused", e=env(GITHUB_SHA="e" * 40), expect=fail("Invalid GitHub checkout or repository identity")),
    sc("a repository identity that differs from the event is refused", ev=event(repository={"id": 5, "full_name": "Semio/Repo"}), expect=fail("Invalid GitHub checkout or repository identity")),
    sc("a repository name that differs from the event is refused", ev=event(repository={"id": REPO_ID, "full_name": "other/repo"}), expect=fail("Invalid GitHub checkout or repository identity")),
    sc("a malformed repository name is refused", e=env(GITHUB_REPOSITORY="../x"), expect=fail("Invalid GitHub checkout or repository identity")),
    sc("a missing run id is refused", e=env(GITHUB_RUN_ID=None), expect=fail("Invalid GitHub identity")),
    sc("a run id with a leading zero is refused", e=env(GITHUB_RUN_ID="07"), expect=fail("Invalid GitHub identity")),
    sc("an event that is not an object is refused", ev=[], expect=fail("Invalid GitHub event")),
    sc("a malformed head is refused", head="XYZ", expect=fail("Invalid CI checkout")),
    sc("an empty branch ref validates everything", e=env(GITHUB_REF="refs/heads/"), expect=ok("all", None, None, "full-requested", None, 0, 0)) if False else sc("an empty branch ref is refused", e=env(GITHUB_REF="refs/heads/"), expect=fail("Invalid GitHub branch")),
    sc("an unavailable run makes history unavailable", run={"status": 404, "body": None}, expect=ok("all", None, None, "no-successful-ancestor", "history-unavailable", 1, 0)),
    sc("a run of another repository makes history unavailable", run={"status": 200, "body": run_body(repository={"id": 3})}, expect=ok("all", None, None, "no-successful-ancestor", "history-unavailable", 1, 0)),
    sc("a run with another id makes history unavailable", run={"status": 200, "body": run_body(id=100)}, expect=ok("all", None, None, "no-successful-ancestor", "history-unavailable", 1, 0)),
    sc("a failing history page makes history unavailable", pages=[page([], status=403)], expect=ok("all", None, None, "no-successful-ancestor", "history-unavailable", 2, 0)),
    sc("a malformed page makes history unavailable", pages=[{"status": 200, "body": {"total_count": -1, "workflow_runs": []}}], expect=ok("all", None, None, "no-successful-ancestor", "history-unavailable", 2, 0)),
]
many = lambda start, count: [wf(start + i, start + i, B1 if i % 2 else B2, head_branch="dev") for i in range(count)]
scenarios += [
    sc("a second page is read when the first is full", pages=[page(many(1000, 100), total=150), page([wf(1, 1, B3)], total=150)], ancestors={B3: True}, expect=ok("affected", B3, 1, "last-successful-ancestor", None, 3, 1)),
    sc("history is limited to ten pages", pages=[page(many(i * 1000 + 1, 100), total=5000) for i in range(10)], expect=ok("all", None, None, "no-successful-ancestor", "history-limit", 11, 0)),
    sc("the last page ends the history", pages=[page(many(1, 100), total=100)], expect=ok("all", None, None, "no-successful-ancestor", None, 2, 0)),
    sc("a page that claims more than a hundred runs is invalid", pages=[page(many(1, 101), total=101)], expect=ok("all", None, None, "no-successful-ancestor", "history-unavailable", 2, 0)),
]
json.dump({"version": 1, "head": HEAD, "scenarios": scenarios}, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
print(len(scenarios))
