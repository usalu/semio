"""📝 Authoring aid: writes the recovery argument corpus (corpus.json)."""
import json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"


def row(name, args, kind=None, target=None):
    return {"name": name, "args": args, "kind": kind, "target": target}


c = {"version": 1}
c["cases"] = [
    row("build", ["run", "repo-mcp:build"], "frontend", "build"),
    row("install", ["run", "repo-mcp:install"], "frontend", "install"),
    row("every shared option on build", ["run", "repo-mcp:build", "--skip-cache", "--skip-cache=true", "--skip-remote-cache", "--skip-remote-cache=true", "--output=stream"], "frontend", "build"),
    row("the streamed output spelled in two words", ["run", "repo-mcp:build", "--output", "stream"], "frontend", "build"),
    row("a repeated option", ["run", "repo-mcp:install", "--skip-cache", "--skip-cache"], "frontend", "install"),
    row("a build only option on build", ["run", "repo-mcp:build", "--exclude-dependencies"], "frontend", "build"),
    row("a build only option with a value on build", ["run", "repo-mcp:build", "--exclude-dependencies=true"], "frontend", "build"),
    row("a build only option on install is not recovery", ["run", "repo-mcp:install", "--exclude-dependencies"]),
    row("a build only option on maintenance is not recovery", ["run", "repo:cache-reset-nx", "--exclude-dependencies"]),
    row("preview maintenance", ["run", "repo:cache-retire-inactive-preview"], "maintenance", "preview"),
    row("retire maintenance with an option", ["run", "repo:cache-retire-inactive", "--skip-cache"], "maintenance", "retire"),
    row("the Nx cache reset", ["run", "repo:cache-reset-nx"], "cache-reset", "nx"),
    row("the Nx cache reset with streamed output", ["run", "repo:cache-reset-nx", "--output", "stream"], "cache-reset", "nx"),
    row("another output mode is not recovery", ["run", "repo-mcp:build", "--output", "json"]),
    row("a dangling output option is not recovery", ["run", "repo-mcp:build", "--output"]),
    row("an unknown option is not recovery", ["run", "repo-mcp:build", "--verbose"]),
    row("a stray positional is not recovery", ["run", "repo-mcp:build", "extra"]),
    row("another verb is not recovery", ["build", "repo-mcp:build"]),
    row("no arguments are not recovery", []),
    row("a missing target is not recovery", ["run"]),
    row("a project without a target is not recovery", ["run", "repo-mcp"]),
    row("a three part target is not recovery", ["run", "repo-mcp:build:extra"]),
    row("an unknown project is not recovery", ["run", "other:build"]),
    row("an unknown frontend target is not recovery", ["run", "repo-mcp:test"]),
    row("an unknown maintenance target is not recovery", ["run", "repo:other"]),
    row("an inherited object property is not a maintenance target", ["run", "repo:constructor"]),
    row("the prototype key is not a maintenance target", ["run", "repo:__proto__"]),
    row("a method name is not a reset target", ["run", "repo:toString"]),
    row("the maintenance targets do not belong to the frontend project", ["run", "repo-mcp:cache-reset-nx"]),
    row("the frontend targets do not belong to the repo project", ["run", "repo:build"]),
]
json.dump(c, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
