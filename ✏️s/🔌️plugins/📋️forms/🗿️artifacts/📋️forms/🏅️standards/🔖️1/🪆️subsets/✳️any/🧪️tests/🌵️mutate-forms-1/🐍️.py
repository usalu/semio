#!/usr/bin/env python3
"""📋️ An INDEPENDENT second implementation of the `s.forms.form` document, its eleven typed mutations and its
`.dsl.semio` text carrier, in Python, serving as this case's differential oracle.

**Why a second implementation and not a third-party library.** A `form` document carries its survey INLINE
(`definition.steps[].blocks[]`) beside its submitted `responses` and two composed child handles
(`structure`, `results`). XForms, JSON Schema forms and ODK each model a survey, but none of them models this
document's handle members, reads its `.dsl.semio` carrier, or answers its mutation vocabulary, so a
second implementation written from this subset's own schemas is the reference.

**What it was written from.**

* ``🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`` and ``📝️definition/🔣️.json`` —
  the document's members, the step/question records and which members are optional.
* ``🚪️io/📸️snapshot/📝️text/📖️.grammar.semio`` — the text carrier's grammar.
* the eleven mutation leaf payload schemas under ``🧬️schema/🧬️mutations`` (camelCase members).
* rules 1, 2 and 3 of
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md`.
* the ten committed `(before, mutation, after, outcome)` vectors.

**No Rust was read to write this.** `🦀️.rs` beside this file registers the SUBJECT half only.

**WHAT THIS CASE'S EVIDENCE ACTUALLY COVERS, stated plainly rather than implied.** Nine of the ten committed
vectors pin a DIAGNOSTIC and leave the document byte-identical — a `mutation.no-op` warning, a
`mutation.target-missing` or a `mutation.duplicate-id` refusal — and the reference
derives that diagnostic from the document's own `definition.steps` rather than reading it off the committed
outcome. Only `change-form-title` moves the document: it ADDS the `title` member the before-document does
not carry, and its inverse removes it again. The successful create/delete/move/replace branches below are
therefore implemented but not yet witnessed by a committed vector.

**A CROSS-CASE DIVERGENCE the reference surfaced.** `s.playbook.playbook` is the same shape with the same
verbs, and the two subsets answer the same situation differently: a duplicate step id is a REJECTED
`mutation.duplicate-id` here (`create-step`) and an APPLIED `mutation.no-op` there (`add-step`).
"""

# region 🔖️Imports
import copy
import json
import re

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Vocabulary
REQUIRED = ("schema", "id", "version", "structure", "results", "definition", "responses")
"""🗂️ The members every committed form document carries. `title` is optional: absent until
`change-form-title` writes it, which is what its committed vector exercises."""

MEMBERS = REQUIRED + ("title",)

KINDS = ("create-step", "delete-step", "reorder-step", "rename-step", "change-step-description", "create-block", "delete-block", "move-block-to-step", "replace-block", "change-block-field", "change-form-title")
"""🏷️ Every kind the catalog declares, in its declared order."""


def tag_of(kind):
    """🔤️ The internally tagged `mutation` discriminator of a kind — lowerCamelCase of its words."""
    head, *rest = kind.split("-")
    return head + "".join(word[:1].upper() + word[1:] for word in rest)


TAGS = {kind: tag_of(kind) for kind in KINDS}

NO_OP = "mutation.no-op"
TARGET_MISSING = "mutation.target-missing"
DUPLICATE_ID = "mutation.duplicate-id"
INVARIANT = "mutation.invariant"
"""🚨️ The four diagnostic codes this subset's committed vectors raise."""

REJECTING = (TARGET_MISSING, DUPLICATE_ID, INVARIANT)
"""🚦️ Which of the four refuse the mutation rather than warning about it."""

TYPED_DEFAULTS = {"number": "number", "slider": "number", "boolean": "boolean", "text": "string", "longText": "string", "date": "string", "color": "string", "single": "string", "multi": "strings"}
"""🎯️ The answer type a question kind's `default` must have, from `📝️definition/🔣️.json` and the leaf's description; kinds
not listed answer any value."""
# endregion 🔖️Vocabulary


# region 🔖️Scene
def steps_of(document):
    """📋️ The survey's steps — the part of the document nine of the ten kinds address."""
    return document["definition"]["steps"]


def step_at(steps, identity):
    """🔎️ The index of a step, or `None`."""
    return next((at for at, step in enumerate(steps) if step["id"] == identity), None)


def block_at(step, identity):
    """🔎️ The index of a block inside one step, or `None`."""
    return next((at for at, block in enumerate(step.get("blocks", [])) if block["id"] == identity), None)


def numbers_equal(left, right):
    """🔢 Two payload values compared as the wire compares them: `1` and `1.0` are the same number."""
    if isinstance(left, dict) and isinstance(right, dict):
        return set(left) == set(right) and all(numbers_equal(left[key], right[key]) for key in left)
    if isinstance(left, list) and isinstance(right, list):
        return len(left) == len(right) and all(numbers_equal(one, other) for one, other in zip(left, right))
    if isinstance(left, bool) or isinstance(right, bool):
        return left is right
    if isinstance(left, (int, float)) and isinstance(right, (int, float)):
        return float(left) == float(right)
    return left == right


def located(steps, identity):
    """🔎️ `(step index, block index)` of a question in whichever step holds it, or `None`."""
    for at, step in enumerate(steps):
        held = block_at(step, identity)
        if held is not None:
            return at, held
    return None


def with_field(block, field, value):
    """✏️ A copy of the question with one field set to an absolute value; `null` removes an optional field."""
    block = copy.deepcopy(block)
    set_or_clear(block, field, value)
    return block


def fits(kind, value):
    """🎯️ Whether a question of `kind` can default to `value`."""
    expected = TYPED_DEFAULTS.get(kind)
    if expected == "number":
        return isinstance(value, (int, float)) and not isinstance(value, bool)
    if expected == "boolean":
        return isinstance(value, bool)
    if expected == "string":
        return isinstance(value, str)
    if expected == "strings":
        return isinstance(value, list) and all(isinstance(item, str) for item in value)
    return True


def field_refusal(block, field):
    """🛡️ The Fatal code the question breaks once `field` is set: an inverted range, a step that is not positive, a
    default its kind cannot answer, parameters that are no object, an empty or repeated option value / vector key."""
    if field in ("min", "max") and block.get("min") is not None and block.get("max") is not None and block["min"] > block["max"]:
        return INVARIANT
    if field == "step" and "step" in block and not block["step"] > 0:
        return INVARIANT
    if field == "default" and "default" in block and not fits(block["kind"], block["default"]):
        return INVARIANT
    if field == "params" and "params" in block and not isinstance(block["params"], dict):
        return INVARIANT
    if field in ("options", "fields") and field in block:
        ids = [item["value" if field == "options" else "key"] for item in block[field]]
        if any(identity == "" for identity in ids):
            return INVARIANT
        if len(set(ids)) != len(ids):
            return DUPLICATE_ID
    return None


def placed(items, index):
    """📍️ Where an insertion lands: the requested index clamped to the list, the end when absent."""
    return len(items) if index is None else min(index, len(items))
# endregion 🔖️Scene


# region 🔖️Verbs
def diagnose(kind, payload, steps):
    """🚦️ The diagnostic this kind raises against these steps, derived rather than read off the committed
    outcome. `(None, None)` means the verb applies with nothing to say."""
    if kind == "create-step":
        return (DUPLICATE_ID, [payload["step"]["id"]]) if step_at(steps, payload["step"]["id"]) is not None else (None, None)
    if kind == "delete-step":
        return (None, None) if step_at(steps, payload["id"]) is not None else (TARGET_MISSING, [payload["id"]])
    if kind == "reorder-step":
        at = step_at(steps, payload["id"])
        if at is None:
            return (TARGET_MISSING, [payload["id"]])
        return (NO_OP, None) if at == payload["toIndex"] else (None, None)
    if kind == "rename-step":
        at = step_at(steps, payload["id"])
        if at is None:
            return (TARGET_MISSING, [payload["id"]])
        return (NO_OP, None) if steps[at].get("title") == payload["newTitle"] else (None, None)
    if kind == "change-step-description":
        at = step_at(steps, payload["id"])
        if at is None:
            return (TARGET_MISSING, [payload["id"]])
        return (NO_OP, None) if steps[at].get("description") == payload.get("newDescription") else (None, None)
    if kind == "create-block":
        at = step_at(steps, payload["stepId"])
        return (None, None) if at is not None else (TARGET_MISSING, [payload["stepId"]])
    if kind == "delete-block":
        at = step_at(steps, payload["stepId"])
        if at is None:
            return (TARGET_MISSING, [payload["stepId"]])
        held = block_at(steps[at], payload["id"])
        return (None, None) if held is not None else (TARGET_MISSING, [payload["stepId"], payload["id"]])
    if kind == "move-block-to-step":
        at = step_at(steps, payload["stepId"])
        if at is None:
            return (TARGET_MISSING, [payload["stepId"]])
        if step_at(steps, payload["toStepId"]) is None:
            return (TARGET_MISSING, [payload["toStepId"]])
        held = block_at(steps[at], payload["blockId"])
        if held is None:
            return (TARGET_MISSING, [payload["stepId"], payload["blockId"]])
        unmoved = payload["stepId"] == payload["toStepId"] and held == payload["index"]
        return (NO_OP, None) if unmoved else (None, None)
    if kind == "replace-block":
        at = step_at(steps, payload["stepId"])
        if at is None:
            return (TARGET_MISSING, [payload["stepId"]])
        held = block_at(steps[at], payload["block"]["id"])
        if held is None:
            return (TARGET_MISSING, [payload["stepId"], payload["block"]["id"]])
        return (NO_OP, None) if numbers_equal(steps[at]["blocks"][held], payload["block"]) else (None, None)
    if kind == "change-block-field":
        where = located(steps, payload["blockId"])
        if where is None:
            return (TARGET_MISSING, [payload["blockId"]])
        block = steps[where[0]]["blocks"][where[1]]
        changed = with_field(block, payload["field"], payload["value"])
        if numbers_equal(changed, block):
            return (NO_OP, None)
        refused = field_refusal(changed, payload["field"])
        return (refused, [payload["blockId"]]) if refused is not None else (None, None)
    if kind == "change-form-title":
        return (None, None)
    raise AssertionError("mutate-%s: this implementation declares no verb for that kind" % kind)


def set_or_clear(record, member, value):
    """✏️ Writes an optional member, or removes it when the payload carries `null`."""
    if value is None:
        record.pop(member, None)
    else:
        record[member] = value


def apply_mutation(document, kind, payload):
    """🦠️ Applies one kind to the document: a diagnosed refusal or no-op leaves it untouched, every other
    outcome edits `definition.steps` or the `title` member."""
    document = copy.deepcopy(document)
    if diagnose(kind, payload, steps_of(document))[0] is not None:
        return document
    steps = steps_of(document)
    if kind == "create-step":
        steps.insert(placed(steps, payload.get("index")), copy.deepcopy(payload["step"]))
    elif kind == "delete-step":
        steps.pop(step_at(steps, payload["id"]))
    elif kind == "reorder-step":
        step = steps.pop(step_at(steps, payload["id"]))
        steps.insert(placed(steps, payload["toIndex"]), step)
    elif kind == "rename-step":
        steps[step_at(steps, payload["id"])]["title"] = payload["newTitle"]
    elif kind == "change-step-description":
        set_or_clear(steps[step_at(steps, payload["id"])], "description", payload.get("newDescription"))
    elif kind == "create-block":
        blocks = steps[step_at(steps, payload["stepId"])]["blocks"]
        blocks.insert(placed(blocks, payload.get("index")), copy.deepcopy(payload["block"]))
    elif kind == "delete-block":
        step = steps[step_at(steps, payload["stepId"])]
        step["blocks"].pop(block_at(step, payload["id"]))
    elif kind == "move-block-to-step":
        source = steps[step_at(steps, payload["stepId"])]
        block = source["blocks"].pop(block_at(source, payload["blockId"]))
        target = steps[step_at(steps, payload["toStepId"])]["blocks"]
        target.insert(placed(target, payload["index"]), block)
    elif kind == "replace-block":
        step = steps[step_at(steps, payload["stepId"])]
        step["blocks"][block_at(step, payload["block"]["id"])] = copy.deepcopy(payload["block"])
    elif kind == "change-block-field":
        at, held = located(steps, payload["blockId"])
        steps[at]["blocks"][held] = with_field(steps[at]["blocks"][held], payload["field"], payload["value"])
    elif kind == "change-form-title":
        set_or_clear(document, "title", payload.get("newTitle"))
    return document


def inverse_mutation(document, kind, payload):
    """↩️ The kind's OWN inverse, derived from the BASE document it was applied to. A refused or no-op
    mutation has nothing to undo."""
    steps = steps_of(document)
    if diagnose(kind, payload, steps)[0] is not None:
        return []
    if kind == "create-step":
        return [("delete-step", {"id": payload["step"]["id"]})]
    if kind == "delete-step":
        at = step_at(steps, payload["id"])
        return [("create-step", {"step": steps[at], "index": at})]
    if kind == "reorder-step":
        return [("reorder-step", {"id": payload["id"], "toIndex": step_at(steps, payload["id"])})]
    if kind == "rename-step":
        return [("rename-step", {"id": payload["id"], "newTitle": steps[step_at(steps, payload["id"])]["title"]})]
    if kind == "change-step-description":
        return [("change-step-description", {"id": payload["id"], "newDescription": steps[step_at(steps, payload["id"])].get("description")})]
    if kind == "create-block":
        return [("delete-block", {"stepId": payload["stepId"], "id": payload["block"]["id"]})]
    if kind == "delete-block":
        step = steps[step_at(steps, payload["stepId"])]
        at = block_at(step, payload["id"])
        return [("create-block", {"stepId": payload["stepId"], "block": step["blocks"][at], "index": at})]
    if kind == "move-block-to-step":
        at = block_at(steps[step_at(steps, payload["stepId"])], payload["blockId"])
        return [("move-block-to-step", {"stepId": payload["toStepId"], "blockId": payload["blockId"], "toStepId": payload["stepId"], "index": at})]
    if kind == "replace-block":
        step = steps[step_at(steps, payload["stepId"])]
        return [("replace-block", {"stepId": payload["stepId"], "block": step["blocks"][block_at(step, payload["block"]["id"])]})]
    if kind == "change-block-field":
        at, held = located(steps, payload["blockId"])
        return [("change-block-field", {"blockId": payload["blockId"], "field": payload["field"], "value": steps[at]["blocks"][held].get(payload["field"])})]
    if kind == "change-form-title":
        return [("change-form-title", {"newTitle": document.get("title")})]
    raise AssertionError("inverse-%s: this implementation declares no verb for that kind" % kind)
# endregion 🔖️Verbs


# region 🔖️TextCarrier
ARTIFACT_MARK = "semio forms.form.dsl v1"
"""🔖️ The carrier's first line (`artifact-mark` in the grammar)."""

TEXT, NUMBER, BOOL, VALUE, BLOCK = "text", "number", "bool", "value", "block"
CHILD = {"child_id": TEXT, "target": TEXT}
OPTION = {"value": TEXT, "label": TEXT}
FIELD = {"key": TEXT, "label": TEXT, "value": NUMBER}
QUESTION = {
    "id": TEXT, "label": TEXT, "kind": TEXT, "description": TEXT, "required": BOOL, "placeholder": TEXT, "default": VALUE, "min": NUMBER, "max": NUMBER, "step": NUMBER,
    "unit": TEXT, "text": TEXT, "options": ("list", OPTION), "fields": ("list", FIELD), "schema": TEXT, "src": TEXT, "accept": TEXT, "fixture-slug": TEXT, "params": VALUE, "condition": BLOCK,
}
STEP = {"id": TEXT, "title": TEXT, "description": TEXT, "blocks": ("list", QUESTION)}
ANSWER = {"question-id": TEXT, "label": TEXT, "kind": TEXT, "value": VALUE}
RESPONSE = {"id": TEXT, "submitted-at": NUMBER, "definition-version": TEXT, "answers": ("list", ANSWER)}
DEFINITION = {"steps": ("list", STEP)}
DOCUMENT = {"schema": TEXT, "id": TEXT, "version": TEXT, "title": TEXT, "definition": ("record", DEFINITION), "responses": ("list", RESPONSE), "structure": ("record", CHILD), "results": ("record", CHILD)}
"""📖️ The grammar's records, member by member, with the scalar each member reads as."""

TOKEN = re.compile(r'\s*(?:(?P<string>"(?:[^"\\]|\\.)*")|(?P<punct>[\[\]{}=])|(?P<atom>[^\s\[\]{}="]+))')


def tokenize(text):
    """🔤️ The carrier body as `(kind, text)` tokens: quoted strings, `[ ] { } =`, and bare atoms."""
    tokens, at = [], 0
    while at < len(text):
        match = TOKEN.match(text, at)
        if match is None or match.end() == at:
            if text[at:].strip() == "":
                break
            raise AssertionError("identity-round-trip: unreadable carrier text at offset %d: %r" % (at, text[at:at + 40]))
        at = match.end()
        kind = match.lastgroup
        tokens.append((kind, json.loads(match.group(kind)) if kind == "string" else match.group(kind)))
    return tokens


def camel(key):
    """🐫 A carrier member name (`fixture-slug`, `child_id`) as its document member (`fixtureSlug`, `childId`)."""
    head, *rest = re.split(r"[-_]", key)
    return head + "".join(part[:1].upper() + part[1:] for part in rest)


class Reader:
    """📖️ A spec-driven reader over the token stream: each record reads the members its grammar rule
    names, a record in a list ends where a member it already holds starts again."""

    def __init__(self, tokens):
        self.tokens, self.at = tokens, 0

    def peek(self, offset=0):
        return self.tokens[self.at + offset] if self.at + offset < len(self.tokens) else (None, None)

    def take(self, expected=None):
        token = self.peek()
        if token[0] is None or (expected is not None and token[1] != expected):
            raise AssertionError("identity-round-trip: expected %r, found %r" % (expected, token[1]))
        self.at += 1
        return token

    def scalar(self, kind):
        token_kind, text = self.take()
        if token_kind == "punct":
            raise AssertionError("identity-round-trip: expected a scalar, found %r" % text)
        if kind == TEXT:
            return text
        if kind == BOOL:
            if text not in ("true", "false"):
                raise AssertionError("identity-round-trip: %r is not a boolean" % text)
            return text == "true"
        return float(text)

    def value(self):
        token_kind, text = self.peek()
        if (token_kind, text) == ("punct", "["):
            self.take("[")
            items = []
            while self.peek()[1] != "]":
                items.append(self.entries_until("]") if self.peek(1)[1] == "=" else self.value())
            self.take("]")
            return items
        if (token_kind, text) == ("punct", "{"):
            self.take("{")
            entries = self.entries_until("}")
            self.take("}")
            return entries
        self.take()
        if token_kind == "string":
            return text
        if text in ("true", "false"):
            return text == "true"
        if re.fullmatch(r"-?\d+", text):
            return int(text)
        if re.fullmatch(r"-?(\d+\.\d*|\.\d+|\d+)([eE][-+]?\d+)?", text):
            return float(text)
        return text

    def entries_until(self, closing):
        entries = {}
        while self.peek()[1] not in (closing, None) and self.peek(1)[1] == "=":
            _, key = self.take()
            self.take("=")
            entries[key] = self.value()
        return entries

    def expression(self):
        _, kind = self.take()
        if kind == "const":
            self.take("value")
            self.take("=")
            return {"kind": "const", "value": self.value()}
        if kind == "var":
            self.take("name")
            self.take("=")
            return {"kind": "var", "name": self.scalar(TEXT)}
        if kind == "eq":
            return {"kind": "eq", "left": self.nested("left"), "right": self.nested("right")}
        if kind == "truthy":
            return {"kind": "truthy", "expr": self.nested("expr")}
        if kind in ("and", "or"):
            self.take("items")
            self.take("{")
            items = []
            while self.peek()[1] != "}":
                items.append(self.expression())
            self.take("}")
            return {"kind": kind, "items": items}
        raise AssertionError("identity-round-trip: unknown condition expression %r" % kind)

    def nested(self, name):
        self.take(name)
        self.take("{")
        inner = self.expression()
        self.take("}")
        return inner

    def member(self, shape):
        if shape == BLOCK:
            self.take("{")
            inner = None if self.peek()[1] == "}" else self.expression()
            self.take("}")
            return inner
        self.take("=")
        if isinstance(shape, tuple) and shape[0] == "list":
            self.take("[")
            records = []
            while self.peek()[1] != "]":
                records.append(self.record(shape[1]))
            self.take("]")
            return records
        if isinstance(shape, tuple):
            return self.record(shape[1])
        return self.value() if shape == VALUE else self.scalar(shape)

    def record(self, spec):
        document = {}
        while True:
            _, key = self.peek()
            follows = self.peek(1)[1]
            if key not in spec or camel(key) in document or follows not in ("=", "{") or (follows == "{") != (spec[key] == BLOCK):
                break
            self.take()
            read = self.member(spec[key])
            if read is not None:
                document[camel(key)] = read
        if not document:
            raise AssertionError("identity-round-trip: an empty record at token %d (%r)" % (self.at, self.peek()[1]))
        return document


def child_handle(record):
    """🧩️ A composed child handle as the document carries it: the inline `target` URI
    `<artifactId>!<artifactKind>@<standard>/<subset>` expanded into its parts."""
    match = re.fullmatch(r"([^!]+)!([^@]+)@([^/]+)/(.+)", record["target"])
    if match is None:
        raise AssertionError("identity-round-trip: %r is not a child target URI" % record["target"])
    artifact, kind, standard, subset = match.groups()
    return {"childId": record["childId"], "target": {"artifactId": artifact, "dialect": {"artifactKind": kind, "standard": standard, "subset": subset}}}


def read_carrier(text):
    """📥️ Reads a `.dsl.semio` form document into the document the committed JSON vectors spell."""
    head, _, body = text.partition("\n")
    if head.strip() != ARTIFACT_MARK:
        raise AssertionError("identity-round-trip: the carrier starts with %r, not %r" % (head, ARTIFACT_MARK))
    reader = Reader(tokenize(body))
    document = reader.record(DOCUMENT)
    if reader.peek()[0] is not None:
        raise AssertionError("identity-round-trip: unread carrier text from token %r on" % (reader.peek()[1],))
    for member in ("structure", "results"):
        document[member] = child_handle(document[member])
    for response in document.get("responses", []):
        if "submittedAt" in response:
            response["submittedAt"] = int(response["submittedAt"])
    return document
# endregion 🔖️TextCarrier


# region 🔖️Laws
def declared(outcome):
    """🚨️ The (status, code, path) a committed `🎯️outcome` vector declares."""
    listed = [message.get("code") for message in outcome.get("messages", []) if message.get("code")]
    code = listed[0] if listed else outcome.get("code")
    return outcome.get("status"), code, outcome.get("path")


def diagnoses_as_committed(kind, produced, outcome):
    """⚖️ The derived diagnostic against the committed one — status, code and path."""
    status, code, path = declared(outcome)
    derived_code, derived_path = produced
    derived_status = "rejected" if derived_code in REJECTING else "no-op" if derived_code == NO_OP else "applied"
    if (derived_status, derived_code) != (status, code):
        raise AssertionError("mutate-%s: this implementation derives %r/%r, the committed 🎯️outcome vector declares %r/%r" % (kind, derived_status, derived_code, status, code))
    if derived_path is not None and path is not None and derived_path != path:
        raise AssertionError("mutate-%s: this implementation derives the path %r, the committed vector declares %r" % (kind, derived_path, path))


def equals_committed(kind, produced, committed):
    """🎯️ The committed after-document claim, member by member, with no tolerance and no ignored key."""
    for member in sorted(set(produced) | set(committed)):
        if not numbers_equal(produced.get(member, "⌀"), committed.get(member, "⌀")):
            raise AssertionError("mutate-%s: %s is %s, the committed after-document says %s" % (kind, member, json.dumps(produced.get(member), sort_keys=True)[:300], json.dumps(committed.get(member), sort_keys=True)[:300]))


def restores(kind, restored, original):
    """↩️ The full inverse law, member for member."""
    for member in sorted(set(restored) | set(original)):
        if not numbers_equal(restored.get(member, "⌀"), original.get(member, "⌀")):
            raise AssertionError("inverse-%s: %s came back as %s, not %s" % (kind, member, json.dumps(restored.get(member), sort_keys=True)[:300], json.dumps(original.get(member), sort_keys=True)[:300]))


def validate(document, where):
    """✅️ Holds a document to the shape the schemas state: the seven always-present members, `title`
    only beyond them, two well-formed child handles, and every step and question carrying its identity."""
    if not set(REQUIRED) <= set(document):
        raise AssertionError("%s: a form document must carry %r, found %r" % (where, sorted(REQUIRED), sorted(document)))
    if not set(document) <= set(MEMBERS):
        raise AssertionError("%s: a form document may carry only %r, found %r" % (where, sorted(MEMBERS), sorted(document)))
    for member in ("structure", "results"):
        if set(document[member]) != {"childId", "target"} or set(document[member]["target"]) != {"artifactId", "dialect"}:
            raise AssertionError("%s: the composed %s child handle must carry childId and an expanded target, found %r" % (where, member, document[member]))
    if set(document["definition"]) != {"steps"}:
        raise AssertionError("%s: the definition carries only steps, found %r" % (where, sorted(document["definition"])))
    for step in steps_of(document):
        if not {"id", "title", "blocks"} <= set(step) or not set(step) <= {"id", "title", "description", "blocks"}:
            raise AssertionError("%s: step %r must carry id, title and blocks, optionally a description" % (where, step.get("id")))
        for block in step["blocks"]:
            if not {"id", "label", "kind"} <= set(block) or not set(block) <= {camel(name) for name in QUESTION}:
                raise AssertionError("%s: question %r carries %r" % (where, block.get("id"), sorted(block)))
# endregion 🔖️Laws


# region 🔖️Plan
def doc_json(ctx):
    """📜️ The scenario's doc string — the Python `Context` has no accessor of its own."""
    for step in ctx.scenario["steps"]:
        if step.get("docString"):
            return json.loads(step["docString"])
    raise AssertionError("scenario %s carries no doc string" % ctx.scenario["id"])


def uri_in(ctx, needle):
    """🧫️ The one declared fixture URI of this scenario's steps containing `needle`."""
    for step in ctx.scenario["steps"]:
        for token in step["text"].split():
            if token.startswith(("asset://", "shared://")) and needle in token:
                return token
    raise AssertionError("scenario %s declares no fixture URI containing %r" % (ctx.scenario["id"], needle))


def json_fixture(ctx, needle):
    """🧫️ The declared JSON fixture this scenario names."""
    return json.loads(ctx.fixture_bytes(uri_in(ctx, needle)).decode("utf-8"))


def payload_of(ctx, kind):
    """🦠️ The committed payload, checked to carry this kind's own internally tagged discriminator."""
    payload = json_fixture(ctx, "🦠️mutation")
    if payload.get("mutation") != TAGS[kind]:
        raise AssertionError("%s: the committed vector carries a %r payload, not %r" % (ctx.scenario["id"], payload.get("mutation"), TAGS[kind]))
    return {key: value for key, value in payload.items() if key != "mutation"}


def before_of(ctx, kind, where):
    """⬅️ The committed before-document, whose steps must be the scene the feature row states."""
    spec = doc_json(ctx)
    if spec.get("kind") != kind:
        raise AssertionError("%s: the feature's doc string states %r" % (where, spec.get("kind")))
    before = json_fixture(ctx, "⬅️before")
    validate(before, where)
    if spec.get("scene") != steps_of(before):
        raise AssertionError("%s: the feature row's scene %s is not the committed before-document's steps %s" % (where, json.dumps(spec.get("scene"))[:200], json.dumps(steps_of(before))[:200]))
    return before


def outcome_of(payload):
    """📤️ Wraps a projection with its own compact serialization as the raw artifact."""
    return Outcome(payload, raw=json.dumps(payload, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))
# endregion 🔖️Plan


# region 🔖️Handlers
def mutate_handler(kind):
    """🎯️ Derives this kind's diagnostic from the document, asserts it against the committed outcome, and
    answers the document the verb leaves behind."""

    def handler(ctx):
        where = "mutate-%s" % kind
        before = before_of(ctx, kind, where)
        after = json_fixture(ctx, "➡️after")
        outcome = json_fixture(ctx, "🎯️outcome")
        payload = payload_of(ctx, kind)
        diagnoses_as_committed(kind, diagnose(kind, payload, steps_of(before)), outcome)
        applied = apply_mutation(before, kind, payload)
        validate(applied, where)
        equals_committed(kind, applied, after)
        return outcome_of(applied)

    return handler


def inverse_handler(kind):
    """↩️ Applies one kind and then its OWN computed inverse and requires the committed before-document back,
    member for member."""

    def handler(ctx):
        before = before_of(ctx, kind, "inverse-%s" % kind)
        payload = payload_of(ctx, kind)
        current = apply_mutation(before, kind, payload)
        for step_kind, step_payload in inverse_mutation(before, kind, payload):
            current = apply_mutation(current, step_kind, step_payload)
        restores(kind, current, before)
        return outcome_of(current)

    return handler


def identity_handler(ctx):
    """🔁️ Reads the real committed `.dsl.semio` artifact through this implementation's own carrier reader and
    answers the document it holds. In role it also requires the document to be one this subset accepts and
    to exercise the survey records, the child handles and a nested value."""
    committed = ctx.fixture_bytes(uri_in(ctx, "🗣️.dsl.semio")).decode("utf-8")
    document = read_carrier(committed)
    validate(document, "identity-round-trip")
    blocks = [block for step in steps_of(document) for block in step["blocks"]]
    if not blocks or not any("params" in block or "options" in block for block in blocks):
        raise AssertionError("identity-round-trip: the committed artifact must carry questions with nested options or params, or it would not exercise the carrier's value grammar")
    return outcome_of(document)
# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Registration by FULL expanded scenario id, in the ORACLE role only — registering these handlers as
    subjects too would make the reference its own subject and manufacture a green self-comparison."""
    built = Adapter("python")
    for kind in KINDS:
        built = built.oracle("mutate-%s" % kind, mutate_handler(kind))
        built = built.oracle("inverse-%s" % kind, inverse_handler(kind))
    return built.oracle("identity-round-trip", identity_handler)
# endregion 🔖️Registration
