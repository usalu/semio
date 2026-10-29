"""Independent model of the graph phase clock: replays a fixture case and prints the published resources, one JSON list per case.

Usage: python3 model.py fixture.json"""
import json, sys, urllib.parse


def quote(text):
    return urllib.parse.quote(text, safe="-_.!~*'()")


def replay(case):
    options = {"maximumNames": 3, "secondsStep": 5, "maximumResourceCharacters": 480, "maximumNameCharacters": 96, **case.get("options", {})}
    minimum, phases, published = case["minimumMilliseconds"], {}, []

    def resource(phase, state, done):
        return "build:graph-phase:%s:%d:%d%s" % (phase, done, state["total"], state["detail"])

    def emit(phase, state, done, at):
        state["lastAt"], state["lastDone"] = at, done
        state["lastResource"] = resource(phase, state, done)
        published.append(state["lastResource"])

    def detail(phase, total, running):
        if not running:
            return ""
        size = len("build:graph-phase:%s:%d:%d" % (phase, total, total))
        seconds = int(max(0, running[0]["seconds"]) // options["secondsStep"]) * options["secondsStep"]
        size += len(":%d:%d:" % (len(running), seconds))
        names = []
        for row in running[: options["maximumNames"]]:
            text = quote(row["name"][: options["maximumNameCharacters"]])
            extra = len(text) + (1 if names else 0)
            if size + extra > options["maximumResourceCharacters"]:
                break
            names.append(text)
            size += extra
        return ":%d:%d:%s" % (len(running), seconds, ",".join(names))

    for step in case["steps"]:
        at, phase = step["at"], step["phase"]
        if step["op"] == "begin":
            state = {"total": step["total"], "done": 0, "startedAt": at, "lastAt": float("-inf"), "lastDone": -1, "detail": "", "lastResource": ""}
            phases[phase] = state
            emit(phase, state, 0, at)
        elif step["op"] == "advance":
            state = phases[phase]
            state["done"] = min(step["done"], state["total"])
            if (state["done"] == state["lastDone"] and resource(phase, state, state["done"]) == state["lastResource"]) or (state["done"] != state["total"] and at - state["lastAt"] < minimum):
                continue
            emit(phase, state, state["done"], at)
        elif step["op"] == "running":
            state = phases[phase]
            state["detail"] = detail(phase, state["total"], step["running"])
            if resource(phase, state, state["done"]) == state["lastResource"] or at - state["lastAt"] < minimum:
                continue
            emit(phase, state, state["done"], at)
        else:
            state = phases[phase]
            state["detail"] = ""
            if state["lastDone"] != state["done"] or resource(phase, state, state["done"]) != state["lastResource"]:
                emit(phase, state, state["done"], at)
            del phases[phase]
    return published


if __name__ == "__main__":
    fixture = json.load(open(sys.argv[1], encoding="utf8"))
    print(json.dumps([replay(case) for case in fixture["cases"]], ensure_ascii=False))
