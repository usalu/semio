"""📝 Authoring aid: writes the host capacity argument corpus (corpus.json): systematic argument vectors for the capacity ask and the granted-jobs rewrite."""
import itertools, json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"
operations = ["run", "run-many", "affected", "exec", "watch", "cargo", "show", "graph", "dependencies", "command", "bogus", "--help"]
parallel_styles = [[], ["--parallel", "4"], ["--parallel=5"], ["--parallel", "-1"], ["--parallel"], ["--parallel", "abc"], ["--parallel=0"], ["--parallel=2.5"], ["--parallel=007"], ["--parallel", "3", "--parallel=6"], ["--parallelism=7"], ["--parallelism", "8"]]
target_styles = [[], ["--targets=build,test"], ["-t", "lint"], ["--target", "test"], ["--targets=,build,"], ["-t=generate-x"], ["--target=service,e2e-x"], ["--targets"], ["--targets", "-x"], ["run-me:test"]]
environments = [{}, {"SEMIO_BUILD_PARALLEL": "3"}, {"SEMIO_BUILD_PARALLEL": "abc"}, {"SEMIO_BUILD_PARALLEL": ""}]
rows = []
for operation, parallel, targets, environment in itertools.product(operations[:6], parallel_styles[:6], target_styles[:5], environments[:3]):
    rows.append({"args": [operation, *parallel, *targets], "environment": environment})
for operation in operations:
    for parallel in parallel_styles:
        rows.append({"args": [operation, *parallel], "environment": {"SEMIO_BUILD_PARALLEL": "2"}})
        rows.append({"args": [operation, *parallel, "--", "--parallel=9", "-t", "build"], "environment": {}})
        rows.append({"args": [operation, *parallel, "--help"], "environment": {}})
        rows.append({"args": [operation, "-h", *parallel], "environment": {}})
for target in ["x:test", "x:test:ci", "x", "a:b:c", "x:", "x:smoke-all", "x:typecheck", "x:format-check", "x:lint-fix", "x:generate", "x:codegenerator", "x:build", "x:publish-x", "x:dev", "x:preview"]:
    rows.append({"args": ["run", target, "--parallel=2"], "environment": {}})
rows.append({"args": ["run"], "environment": {"SEMIO_BUILD_PARALLEL": "2"}})
rows.append({"args": [], "environment": {}})
granted = []
for args in [["run", "x:test"], ["run", "x:test", "--parallel", "9"], ["run", "x:test", "--parallel=9", "--other"], ["run", "x:test", "--parallel"], ["run", "--parallel", "1", "--parallel=2", "y", "--", "--parallel=7"], ["cargo", "check", "--parallelism=3", "--", "--parallel=4"], ["cargo", "check", "--parallel=3"], ["run", "x:test", "--parallelism=3"], ["run", "x:test", "--", "a", "b"], [], ["--parallel=1", "--"]]:
    for operation, jobs in [("run", 1), ("run", 8), ("cargo", 2), ("run-many", 3)]:
        granted.append({"args": args, "operation": operation, "jobs": jobs})
commands = []
memories = [[], ["--memory=0"], ["--memory=abc"], ["--memory", "-5"], ["--memory=2048"], ["--memory", "--cpu=2"]]
cpus = [[], ["--cpu=0"], ["--cpu", "3"]]
jobs_styles = [[], ["--jobs=0"], ["--jobs", "z"]]
operations_cmd = [[], ["--operation=nope"], ["--operation="]]
for lead in (["run"], ["status"], ["bogus"], []):
    for memory, cpu, jobs_style, operation, tail in itertools.product(memories, cpus, jobs_styles, operations_cmd, ([], ["--", "x"], ["--", "x", "--memory=0"])):
        if lead != ["run"] and (memory or cpu or jobs_style or operation or tail):
            continue
        commands.append(lead + memory + cpu + jobs_style + operation + tail)
c = {"version": 1, "asks": rows, "granted": granted, "commands": commands, "signals": ["SIGINT", "SIGTERM"]}
c["authored"] = [
    {"name": "explicit parallel and target", "args": ["run", "x:typecheck", "--parallel=6"], "environment": {}, "expect": {"operation": "run", "preferredJobs": 6, "kinds": ["check"]}},
    {"name": "the environment fills in when no option is given", "args": ["run-many", "--targets=build,test"], "environment": {"SEMIO_BUILD_PARALLEL": "3"}, "expect": {"operation": "run-many", "preferredJobs": 3, "kinds": ["build", "test"]}},
    {"name": "cargo ignores the build environment parallelism", "args": ["cargo", "--parallelism=4"], "environment": {"SEMIO_BUILD_PARALLEL": "9"}, "expect": {"operation": "cargo", "preferredJobs": 4, "kinds": []}},
    {"name": "light commands ask for nothing", "args": ["--help"], "environment": {}, "expect": None},
    {"name": "help anywhere before the separator asks for nothing", "args": ["run", "x:test", "-h"], "environment": {}, "expect": None},
    {"name": "help after the separator is the child's", "args": ["run", "x:test", "--parallel=2", "--", "--help"], "environment": {}, "expect": {"operation": "run", "preferredJobs": 2, "kinds": ["test"]}},
]
json.dump(c, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False)
print(len(rows), len(granted))
