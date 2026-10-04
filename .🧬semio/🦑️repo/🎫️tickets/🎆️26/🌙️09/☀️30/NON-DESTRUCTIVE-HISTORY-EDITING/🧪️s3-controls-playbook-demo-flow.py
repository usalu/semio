"""🌊️ S3-CONTROLS §20.15 — moves the playbook demo's steps out of its parent DSL into its `flow` child.

Reads the committed parent asset (`steps="[…]"` JSON inside the playbook DSL), writes the steps as the child's own
`s.stdio.semio@v1/flow` text (one `step` node per step: label = title, params `blocksJson` and `description`, position
index × 220; consecutive steps chained by `sequence` edges `seq-<a>-<b>` from port `next` to port `prev`), and rewrites the
parent asset to name the child by the fixed id `playbook-demo-flow`. Idempotent: a parent without `steps=` is left alone.
"""
import json
import pathlib
import re

ROOT = pathlib.Path(__file__).resolve().parents[7]
ASSETS = ROOT / "✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🎬️demo"
PARENT = ASSETS / "🗣️.dsl.semio"
CHILD = ASSETS / "🌊️flow/🗣️.dsl.semio"
FLOW_ID = "playbook-demo-flow"


def unquote(text, start):
    out, index = [], start
    while text[index] != '"':
        if text[index] == "\\":
            index += 1
        out.append(text[index])
        index += 1
    return "".join(out), index + 1


def hexed(value):
    return value.encode("utf-8").hex()


def number(value):
    return str(int(value)) if float(value).is_integer() else repr(float(value))


def main():
    text = PARENT.read_text()
    if 'steps="' not in text:
        return
    header, body = text.split("\n", 1)
    steps_json, after = unquote(body, body.index('steps="') + len('steps="'))
    steps = json.loads(steps_json)
    nodes = []
    for index, step in enumerate(steps):
        params = [("blocksJson", json.dumps(step.get("blocks", []), separators=(",", ":"), ensure_ascii=False))]
        if step.get("description") is not None:
            params.append(("description", step["description"]))
        param_text = ",".join(f"[{hexed(key)},{hexed(value)}]" for key, value in params)
        nodes.append(f"[{hexed(step['id'])},{hexed('step')},{hexed(step['title'])},[{param_text}],[{number(index * 220)},0]]")
    edges = [f"[{hexed('seq-' + a['id'] + '-' + b['id'])},[{hexed(a['id'])},{hexed('next')}],[{hexed(b['id'])},{hexed('prev')}],{hexed('sequence')}]" for a, b in zip(steps, steps[1:])]
    CHILD.parent.mkdir(parents=True, exist_ok=True)
    CHILD.write_text(f"semio stdio.semio.flow.dsl v1\nschema={hexed('stdio.semio.flow')}\nnodes=[{','.join(nodes)}]\nedges=[{','.join(edges)}]\n")
    scalars = body[: body.index(' steps="')]
    assert re.fullmatch(r'schema=\S+ id=\S+ version="[^"]*" title="[^"]*"', scalars), scalars
    PARENT.write_text(f'{header}\n{scalars} flow=child_id={FLOW_ID} target="{FLOW_ID}!s.stdio.semio@v1/flow"\n')
    print(f"{len(steps)} steps, {len(edges)} chain edges")


main()
