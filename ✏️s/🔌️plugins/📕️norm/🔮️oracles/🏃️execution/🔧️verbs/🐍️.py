"""🔧️ The closed verb table: apply, refuse and invert rules for every mutation kind."""

from __future__ import annotations

# region 🔖️Imports
import copy
import json

from semio_norm_vocabulary_document import (
    normalised,
    new_value_arguments,
    named_field,
    key_of,
    is_composed_slot,
    collection_key,
    facet_slot,
    facet_entity,
    place,
    kebab,
    qualified_field,
    selected_field,
    identified_slot,
    qualified_slot,
    indexed_collection,
    positions,
    descent,
    nested_arguments,
    nested_kind,
    plural,
    find_container,
    duplicated,
    member_slot,
    target_slot,
    derived_view,
    MISSING,
    rebuild_derived,
    Refused,
)
# endregion 🔖️Imports


# region 🔖️Declared
#: 🧭 The collection path and selectors each kind declares (`edit` of the mutation taxonomy row, read from the artifact's edit rules), keyed by kind; empty until [`declare_kinds`] ran.
DECLARED = {}

#: 🧭 The declared set-field kinds (`edit.rule == "set-field"`), keyed by kind; applied first when their path resolves in the document.
SETS = {}


def declare_kinds(rows):
    """🧭 Declares the kinds of one artifact from its taxonomy rows; a kind with a declared insert, remove or id-removal rule resolves its collection from the declaration, never from its name."""
    for row in rows:
        if row.get("edit") and "rule" not in row["edit"]:
            raise ValueError("kind %s of %s declares an edit without a `rule` (stale taxonomy row; regenerate the mutation-leaf taxonomy from the artifact's edit rules): %s" % (row.get("kind"), row.get("artifact"), sorted(row["edit"])))
    DECLARED.clear()
    SETS.clear()
    SETS.update({row["kind"]: row["edit"] for row in rows if row.get("edit") and row["edit"]["rule"] == "set-field"})
    DECLARED.update({row["kind"]: row["edit"] for row in rows if row.get("edit") and row["edit"]["rule"] != "set-field"})


#: 📚 The rolled-section catalogue by normalised designation (see `📚️catalogue`); empty until [`declare_catalogue`] ran.
CATALOGUE = {}


def declare_catalogue(sections):
    """📚 Declares the shared section catalogue: a record whose `designation` is written to a catalogued one takes the catalogue geometry with it."""
    CATALOGUE.clear()
    CATALOGUE.update(sections)


def catalogued(before, after):
    """📚 `after` with every record whose `designation` changed to a catalogued one replaced by the catalogue record (an uncatalogued designation keeps the stored geometry), like `SteelSection::from_catalogue`."""
    if isinstance(after, dict):
        out = {key: catalogued(before.get(key) if isinstance(before, dict) else None, value) for key, value in after.items()}
        designation = out.get("designation")
        known = CATALOGUE.get("".join(designation.split()).upper()) if isinstance(designation, str) else None
        old = before.get("designation") if isinstance(before, dict) else None
        return copy.deepcopy(known) if known is not None and designation != old else out
    if isinstance(after, list):
        return [catalogued(before[at] if isinstance(before, list) and at < len(before) else None, value) for at, value in enumerate(after)]
    return after


def declared_row(items, selector, arguments, kind):
    """🎯 The row one declared selector picks out of `items`: by the row id the payload key carries, by the position it carries, or the only row for an implied selector."""
    if not isinstance(items, list):
        raise Refused("%s addresses a list this document does not carry" % kind, "mutation.target-missing")
    if selector["kind"] == "implied":
        return items[0] if len(items) == 1 else None
    value = arguments.get(selector["key"])
    if selector["kind"] == "index":
        return items[value] if isinstance(value, int) and 0 <= value < len(items) else None
    return next((row for row in items if isinstance(row, dict) and row.get("id") == value), None)


def declared_list(document, edit, arguments, kind):
    """📚 The list a declared insert or remove path ends in: the path's `[]` segments are consumed by the selectors in order."""
    selectors, node = iter(edit["selectors"]), document
    for segment in edit["path"].split("."):
        name = segment[:-2] if segment.endswith("[]") else segment
        if not isinstance(node, dict) or name not in node:
            raise Refused("%s addresses %r, which this document does not carry" % (kind, edit["path"]), "mutation.target-missing")
        node = node[name]
        if segment.endswith("[]"):
            node = declared_row(node, next(selectors), arguments, kind)
            if node is None:
                raise Refused("%s addresses a parent row of %r this document does not hold" % (kind, edit["path"]), "mutation.target-missing")
    if not isinstance(node, list):
        raise Refused("%s addresses %r, which is not an ordered collection" % (kind, edit["path"]))
    return node


def declared_get(document, kind, arguments):
    """👁️ The current value of a declared set-field."""
    edit, node, selectors = SETS[kind], document, iter(SETS[kind]["selectors"])
    for segment in edit["path"].split(".")[:-1]:
        node = node[segment[:-2] if segment.endswith("[]") else segment]
        if segment.endswith("[]"):
            node = declared_row(node, next(selectors), arguments, kind)
    return node.get(edit["path"].rpartition(".")[2])


def declared_set(document, kind, arguments):
    """✏️ A declared set-field: the path's `[]` segments are consumed by the selectors, the last segment is the field written with the declared value argument."""
    edit, node, selectors = SETS[kind], document, iter(SETS[kind]["selectors"])
    for segment in edit["path"].split(".")[:-1]:
        name = segment[:-2] if segment.endswith("[]") else segment
        if not isinstance(node, dict) or name not in node:
            return None
        node = node[name]
        if segment.endswith("[]"):
            node = declared_row(node, next(selectors), arguments, kind)
    field = edit["path"].rpartition(".")[2]
    if not isinstance(node, dict) or edit["value"] not in arguments:
        return None
    if arguments[edit["value"]] is None:
        node.pop(field, None)
    else:
        node[field] = copy.deepcopy(arguments[edit["value"]])
    return document


def declared_address(edit, arguments):
    """🪪 The parent selector payload of a declared kind."""
    return {selector["key"]: arguments[selector["key"]] for selector in edit["selectors"] if selector["key"] in arguments}


def owned_rows(document, kind, edit, removed):
    """🪢 The rows other top-level lists own through the `owns` declaration of the removal (`studs` keyed by `beamId` for `beams`), as `(partner kind, list name, index, row)` in ascending index order; a list needs a declared insert kind."""
    if not isinstance(removed, dict) or "id" not in removed:
        return []
    owned = []
    for owns in edit.get("owns", []):
        rows = document.get(owns["path"])
        partner = "insert-" + owns["path"].rstrip("s")
        if not isinstance(rows, list) or partner not in DECLARED or DECLARED[partner]["rule"] != "insert-item":
            continue
        owned.extend((partner, owns["path"], at, row) for at, row in enumerate(rows) if isinstance(row, dict) and row.get(owns["key"]) == removed["id"])
    return owned


def declared_apply(document, kind, arguments):
    """⚙️ A declared insert, remove or id-removal on the list its path names."""
    edit = DECLARED[kind]
    items = declared_list(document, edit, arguments, kind)
    if edit["rule"] == "insert-item":
        element = arguments.get(edit["item"])
        if element is None:
            raise Refused("%s carries no element to insert" % kind)
        if duplicated(items, element):
            raise Refused("%s inserts id %r, which this collection already holds" % (kind, element["id"]), "mutation.duplicate-id")
        items.insert(min(arguments.get(edit["index"], len(items)), len(items)), copy.deepcopy(element))
    elif edit["rule"] == "remove-item":
        index = arguments.get(edit["index"])
        if not isinstance(index, int) or isinstance(index, bool) or not 0 <= index < len(items):
            raise Refused("%s addresses element %s of a collection that does not hold it" % (kind, index), "mutation.target-missing")
        removed = items.pop(index)
    else:
        position = next((at for at, row in enumerate(items) if isinstance(row, dict) and row.get("id") == arguments.get(edit["id"])), None)
        if position is None:
            raise Refused("%s addresses a row this document's %r does not hold" % (kind, edit["path"]), "mutation.target-missing")
        removed = items.pop(position)
    if edit["rule"] != "insert-item":
        for _, name, at, _row in reversed(owned_rows(document, kind, edit, removed)):
            document[name].pop(at)
    return document


def declared_partner_kind(kind, prefix):
    """🤝 The declared kind `prefix + <noun>` paired with `kind`, or `None`."""
    partner = prefix + kind.split("-", 1)[1]
    return partner if partner in DECLARED else None


def declared_inverse(document, kind, arguments):
    """↩️ The inverse of a declared insert, remove or id-removal, computed from the base document and addressed exactly as the declared partner kind declares."""
    edit = DECLARED[kind]
    items = declared_list(document, edit, arguments, kind)
    address = declared_address(edit, arguments)
    if edit["rule"] == "insert-item":
        partner = declared_partner_kind(kind, "remove-")
        element = arguments.get(edit["item"])
        if partner is None or not isinstance(element, dict):
            return []
        at = min(arguments.get(edit["index"], len(items)), len(items))
        tail = DECLARED[partner]
        return [(partner, {**address, **({tail["index"]: at} if tail["rule"] == "remove-item" else {tail["id"]: element["id"]})})]
    partner = declared_partner_kind(kind, "insert-")
    if edit["rule"] == "remove-item":
        index = arguments.get(edit["index"])
        position = index if isinstance(index, int) and 0 <= index < len(items) else None
    else:
        position = next((at for at, row in enumerate(items) if isinstance(row, dict) and row.get("id") == arguments.get(edit["id"])), None)
    if partner is None or position is None:
        return []
    tail = DECLARED[partner]
    restore = [(partner, {**address, tail["index"]: position, tail["item"]: copy.deepcopy(items[position])})]
    for owned_partner, _, at, row in owned_rows(document, kind, edit, items[position]):
        restore.append((owned_partner, {DECLARED[owned_partner]["index"]: at, DECLARED[owned_partner]["item"]: copy.deepcopy(row)}))
    return restore
# endregion 🔖️Declared


# region 🔖️Verbs
#: ➕️ The verbs that bring a member into an id-keyed collection or a set-like list; `introduce` is the
#: catalogue vocabulary's spelling of `create`.
ADDS = ("create", "add", "introduce")

#: ➖️ The verbs that take one out again; `retire` is the catalogue vocabulary's spelling of `delete`.
DROPS = ("delete", "remove", "retire")


def replaces_whole(document, verb, arguments):
    """🧱 Whether a setter without a `new<Field>` argument replaces whole slots: always for `update`, whose
    arguments are facets, and for any other setter whose every argument spells a slot of the document —
    `change-materials {materials}` replaces the list."""
    return verb == "update" or (bool(arguments) and all(facet_slot(document, argument, value)[0] is not None for argument, value in arguments.items()))


#: 🫥 The nouns whose records omit a text field holding its empty value (`skip_serializing_if = "String::is_empty"`): writing `""` into such a field drops its key again.
OMITS_EMPTY = ("site",)


def reordering(kind, arguments):
    """🔀️ The `(verb, kind)` a kind is read as: `change-<noun>-order {fromIndex, toIndex}` moves a row of `<noun>s` and is read as `reorder-<noun>`."""
    positional = {normalised(name) for name in arguments} <= {"from", "to", "fromindex", "toindex"}
    if kind.startswith("change-") and kind.endswith("-order") and arguments and positional:
        return "reorder", "reorder-" + kind[len("change-"): -len("-order")]
    return kind.split("-")[0], kind


def field_name(kind, argument, document=None):
    """🎯 The field a new-value argument writes: the named field, or — for the generic `newValue` — the field the kind's noun names (`change-spacing-par {newValue}` writes `spacingPar`); the noun
    may be the head of the record's key (`change-member` writes `memberId`, `change-mass` writes `massKgPerM2`) when exactly one key starts with it."""
    name = named_field(argument)
    if normalised(name) != "value":
        return name
    noun = "-".join(kind.split("-")[1:])
    if isinstance(document, dict) and key_of(document, noun) is None:
        heads = [key for key in document if normalised(key).startswith(normalised(noun))]
        if len(heads) == 1:
            return heads[0]
        tails = [key for key in document if normalised(key).endswith(normalised(noun))]
        if not heads and len(tails) == 1:
            return tails[0]
    return noun


def optional_record(document, kind, name):
    """🫥 The record the kind's first noun locates (`site` of `change-site-en-ground-type`) when `name` is an optional field it does not carry: a record omits a field holding its empty value, so an absent key
    is that field's canonical empty state, and the field is spelled with a lower-case first letter."""
    record = document.get(key_of(document, kind.split("-")[1]) or "") if len(kind.split("-")) > 2 else None
    return (record, name[:1].lower() + name[1:]) if isinstance(record, dict) and key_of(record, name) is None else (None, None)


def keyed_row(document, kind, arguments):
    """🗝️ The `(rows, key, old)` of a keyed upsert `change-<member> {key, value}` inside the record an address descended into (`change-case-number {caseId, key, value}` writes the `numbers` row `key` of the
    case `caseId`, which the descent has already selected); `None` when the kind is not of that shape. `value` is the new value, `null` removes the row."""
    segments = kind.split("-")[1:]
    if kind.split("-")[0] != "change" or len(segments) != 1 or set(arguments) != {"key", "value"} or not isinstance(document, dict):
        return None
    member = key_of(document, plural(segments[0]))
    if member is None or not isinstance(document[member], list):
        raise Refused("%s addresses a member list this document does not carry" % kind)
    rows = document[member]
    return rows, arguments["key"], next((row for row in rows if row.get("id") == arguments["key"]), None)


def apply_mutation(base, kind, arguments, view=MISSING):
    """⚙️ Applies one typed mutation and returns the resulting document, raising [`Refused`] when the
    verb's addressing law cannot be satisfied — an out-of-range index, a field the document has no key
    for, or a composed child slot whose new identity is not derivable from the document alone. Any
    derived mirror the document keeps is rebuilt on the way out, never left stale."""
    return rebuild_derived(derived_view(base) if view is MISSING else view, catalogued(base, apply_verb(copy.deepcopy(base), kind, arguments)) if CATALOGUE else apply_verb(copy.deepcopy(base), kind, arguments), base)


def apply_verb(document, kind, arguments):
    """⚙️ The verb itself, on a document this call owns — inside the record a nested addressing selects."""
    if kind in DECLARED:
        return declared_apply(document, kind, arguments)
    if kind in SETS and declared_set(document, kind, arguments) is not None:
        return document
    steps, record = descent(document, kind, arguments)
    if record is not None:
        apply_verb(record, nested_kind(kind, steps), nested_arguments(arguments, steps))
        return document
    if any(isinstance(value, int) for _, value in steps):
        raise Refused("%s addresses a nested record %s that this document does not hold" % (kind, json.dumps(dict(steps))))
    verb, kind = reordering(kind, arguments)
    updates = new_value_arguments(arguments)
    address = {key: value for key, value in arguments.items() if key not in updates}

    keyed = keyed_row(document, kind, arguments)
    if keyed is not None:
        rows, row_key, old = keyed
        value = arguments["value"]
        if old is not None and value is None:
            rows.remove(old)
        elif old is not None:
            old["value"] = copy.deepcopy(value)
        elif value is not None:
            rows.append({"id": row_key, "value": copy.deepcopy(value)})
        return document

    if verb in ("change", "set", "specify", "update", "rename", "edit", "replace", "resize"):
        if not updates and replaces_whole(document, verb, arguments):
            for argument, value in arguments.items():
                owner, slot = facet_slot(document, argument, value)
                if owner is None:
                    raise Refused("%s replaces the facet %r, which this document neither holds as a field nor as an identified member" % (kind, argument))
                place(owner, slot, value)
            return document
        if not updates:
            raise Refused("%s carries no new-value argument" % kind)
        if "index" in address:
            items = indexed_collection(document, kind, [field_name(kind, argument, document) for argument in updates])
            index = address["index"]
            if not isinstance(items, list) or not isinstance(index, int) or index >= len(items):
                raise Refused("%s addresses element %s of a collection that does not hold it" % (kind, index))
            for argument in updates:
                owner, key = qualified_field(items[index], kind, field_name(kind, argument, document))
                if owner is None:
                    raise Refused("%s names element field %r, which this document has no key for" % (kind, field_name(kind, argument, document)))
                owner[key] = copy.deepcopy(arguments[argument])
            return document
        selected = selected_field(document, kind, address) if len(updates) == 1 else None
        if selected is not None:
            document[selected] = copy.deepcopy(arguments[updates[0]])
            return document
        for argument in updates:
            owner, slot = identified_slot(document, kind, address.get("id"), field_name(kind, argument, document))
            if owner is not None:
                owner[slot] = copy.deepcopy(arguments[argument])
                continue
            key = key_of(document, field_name(kind, argument, document))
            if key is None:
                owner, slot = target_slot(document, kind, address, field_name(kind, argument, document), arguments[argument])
                if owner is None:
                    owner, slot = identified_slot(document, kind, address.get("id"), field_name(kind, argument, document))
                if owner is None:
                    owner, slot = qualified_slot(document, kind, field_name(kind, argument, document), arguments[argument])
                if owner is None:
                    record, spelled = optional_record(document, kind, field_name(kind, argument, document))
                    if record is None:
                        raise Refused("%s names %r, which this document has no key for at its root or under the owner the kind's noun locates" % (kind, field_name(kind, argument, document)))
                    if arguments[argument] not in ("", None):
                        record[spelled] = copy.deepcopy(arguments[argument])
                    continue
                owner[slot] = copy.deepcopy(arguments[argument])
                if arguments[argument] == "" and isinstance(owner, dict) and kind.split("-")[1] in OMITS_EMPTY:
                    owner.pop(slot)
                continue
            if is_composed_slot(document[key]) and not is_composed_slot(arguments[argument]):
                raise Refused("%s would write content into the composed child slot %r, whose childId is content-addressed by a function no specification in this repository states" % (kind, key))
            document[key] = copy.deepcopy(arguments[argument])
        return document

    if verb == "clear":
        key = key_of(document, "-".join(kind.split("-")[1:]))
        if key is None:
            raise Refused("%s names a field this document has no key for" % kind)
        document[key] = None
        return document

    if verb == "insert":
        key = collection_key(document, kind)
        items = document.get(key or "")
        if is_composed_slot(items):
            raise Refused("%s would seed the composed child slot %r, whose childId is content-addressed by a function no specification in this repository states" % (kind, key))
        if not isinstance(items, list):
            raise Refused("%s addresses %r, which is not an ordered collection" % (kind, key))
        element = next((value for name, value in arguments.items() if name != "index" and isinstance(value, (dict, list))), None)
        if element is None:
            element = next((value for name, value in arguments.items() if name != "index"), None)
        if element is None:
            raise Refused("%s carries no element to insert" % kind)
        if duplicated(items, element):
            raise Refused("%s inserts id %r, which this collection already holds" % (kind, element["id"]), "mutation.duplicate-id")
        items.insert(min(arguments.get("index", len(items)), len(items)), copy.deepcopy(element))
        return document

    if verb == "remove":
        key = collection_key(document, kind)
        items = document.get(key or "")
        if is_composed_slot(items):
            raise Refused("%s addresses the composed child slot %r, whose contents this document does not carry" % (kind, key))
        index = arguments.get("index")
        if isinstance(items, list) and isinstance(index, int) and index < len(items) and len(arguments) == 1:
            items.pop(index)
            return document
        return collection_verb(document, kind, arguments)

    if verb == "reorder":
        key = collection_key(document, kind)
        items = document.get(key or "")
        if is_composed_slot(items):
            raise Refused("%s addresses the composed child slot %r, whose contents this document does not carry" % (kind, key))
        source, target = (arguments.get(name) for name in positions(arguments))
        if not isinstance(items, list) or not isinstance(source, int) or not isinstance(target, int) or source >= len(items) or target >= len(items):
            raise Refused("%s addresses positions %s→%s of a collection that does not hold them" % (kind, source, target))
        items.insert(target, items.pop(source))
        return document

    if verb in ADDS + DROPS:
        return collection_verb(document, kind, arguments)

    raise AssertionError("%s: this implementation does not implement the verb this kind declares" % kind)


def owned_collection(document, kind, arguments):
    """📦 The `(collection, element_argument)` a rule-2 or rule-4 verb addresses: the collection the
    kind's noun names outright, or — when the noun is `<owner>-<member>` — the member collection inside
    the owner the address selects."""
    segments = kind.split("-")[1:]
    noun = "-".join(segments)
    parent, key = find_container(document, plural(noun))
    if parent is None:
        parent, key = find_container(document, noun)
    if parent is not None:
        return parent[key], noun
    for cut in range(len(segments) - 1, 0, -1):
        head = "-".join(segments[:cut])
        for spelled in (plural(head), head):
            parent, key = find_container(document, spelled)
            if parent is not None and isinstance(parent[key], list) and parent[key] and all(isinstance(row, dict) and "id" in row for row in parent[key]) and cut > 1:
                return parent[key], noun
    for cut in range(1, len(segments)):
        parent, key = find_container(document, plural("-".join(segments[cut:])))
        if parent is not None and isinstance(parent[key], list):
            return parent[key], noun
    for cut in range(len(segments) - 1, 0, -1):
        owner_name, member_name = "-".join(segments[:cut]), "-".join(segments[cut:])
        parent, key = find_container(document, plural(owner_name))
        if parent is None:
            parent, key = find_container(document, owner_name)
        if parent is None:
            continue
        owner = parent[key]
        address = {name: value for name, value in arguments.items() if name in ("id", "key", "sheet") or normalised(name).endswith("id")}
        if not isinstance(owner, list) and not (isinstance(owner, dict) and key_of(owner, plural(member_name)) is None and key_of(owner, member_name) is None):
            pass
        if isinstance(owner, dict) and (key_of(owner, plural(member_name)) or key_of(owner, member_name)):
            inner = key_of(owner, plural(member_name)) or key_of(owner, member_name)
            return owner[inner], member_name
        container, member_key = member_slot(owner, address)
        if container is None:
            continue
        member = container[member_key]
        inner = key_of(member, plural(member_name)) or key_of(member, member_name) if isinstance(member, dict) else None
        if inner is not None:
            return member[inner], member_name
    return None, None


def collection_verb(document, kind, arguments):
    """➕➖ `create`/`delete` on an id-keyed collection and `add`/`remove` on a set-like member list —
    derivation rule 2's and rule 4's verbs, addressed by the format's own native key."""
    verb = kind.split("-")[0]
    collection, noun = owned_collection(document, kind, arguments)
    if collection is None:
        raise Refused("%s addresses a collection this document does not carry" % kind)
    if verb in ADDS:
        element = next((value for name, value in arguments.items() if normalised(name) == normalised(noun)), None)
        if element is None:
            element = next((value for name, value in arguments.items() if isinstance(value, (dict, list))), None)
        if element is None:
            raise Refused("%s carries no element to add" % kind)
        if duplicated(collection, element):
            raise Refused("%s adds id %r, which this collection already holds" % (kind, element["id"]), "mutation.duplicate-id")
        if isinstance(collection, list):
            index = arguments.get("index")
            collection.insert(min(index, len(collection)) if isinstance(index, int) else len(collection), copy.deepcopy(element))
        elif isinstance(collection, dict):
            identity = element.get("id") if isinstance(element, dict) else None
            if identity is None:
                raise Refused("%s adds to a keyed collection but its element carries no id" % kind)
            collection[identity] = copy.deepcopy(element)
        else:
            raise Refused("%s addresses %r, which is not a collection" % (kind, noun))
        return document
    address = {name: value for name, value in arguments.items() if name != "index" or isinstance(collection, list)}
    container, member_key = member_slot(collection, address)
    if container is None:
        raise Refused("%s addresses a member this document's %r does not hold" % (kind, noun), "mutation.target-missing")
    if isinstance(container, dict):
        del container[member_key]
    else:
        container.pop(member_key)
    return document


def inverse_mutation(kinds, document, kind, arguments):
    """↩️ The mutation's own inverse, always computed from the BASE document, and empty when the target
    is missing — both stated by the addressing convention. A nested kind inverts inside the record its
    addressing selects and every inverse step keeps that same addressing."""
    if kind in DECLARED:
        return declared_inverse(document, kind, arguments)
    if kind in SETS and declared_set(copy.deepcopy(document), kind, arguments) is not None:
        old = declared_get(document, kind, arguments)
        return [(kind, {**arguments, SETS[kind]["value"]: copy.deepcopy(old)})]
    steps, record = descent(document, kind, arguments)
    if record is not None:
        inner = nested_kind(kind, steps)
        prefix = kind.split("-", 1)[1][: len(kind.split("-", 1)[1]) - len(inner.split("-", 1)[1])]
        outer = lambda step: next((candidate for candidate in (step.replace("-", "-" + prefix, 1), step) if candidate in kinds), step)
        return [(outer(step), {**dict(steps), **payload}) for step, payload in inverse_mutation([nested_kind(candidate, steps) for candidate in kinds], record, inner, nested_arguments(arguments, steps))]
    if any(isinstance(value, int) for _, value in steps):
        return []
    original = kind
    verb, kind = reordering(kind, arguments)
    updates = new_value_arguments(arguments)
    address = {key: value for key, value in arguments.items() if key not in updates}
    noun = "-".join(kind.split("-")[1:])

    keyed = keyed_row(document, kind, arguments)
    if keyed is not None:
        rows, row_key, old = keyed
        if old is None and arguments["value"] is None:
            return []
        return [(kind, {**arguments, "value": copy.deepcopy(old["value"]) if old is not None else None})]

    if verb in ("change", "set", "specify", "update", "rename", "edit", "replace", "resize"):
        if not updates and replaces_whole(document, verb, arguments):
            restored, drops = {}, []
            for argument, value in arguments.items():
                owner, slot = facet_slot(document, argument, value)
                if owner is None:
                    return []
                if isinstance(owner, list) and slot == len(owner):
                    partner = declared_partner(kinds, DROPS, kebab(facet_entity(argument)))
                    if partner is None:
                        return []
                    drops.append((partner, {"index": slot} if partner.startswith("remove-") else {"id": value["id"]}))
                    continue
                restored[argument] = copy.deepcopy(owner[slot])
            return ([(kind, restored)] if restored else []) + drops
        source = document
        if "index" in address:
            items = indexed_collection(document, kind, [field_name(kind, argument, document) for argument in updates])
            index = address["index"]
            if not isinstance(items, list) or not isinstance(index, int) or index >= len(items):
                return []
            restored = dict(address)
            for argument in updates:
                owner, key = qualified_field(items[index], kind, field_name(kind, argument, document))
                if owner is None:
                    return []
                restored[argument] = copy.deepcopy(owner[key])
            return [(kind, restored)]
        restored = dict(address)
        selected = selected_field(source, kind, address) if len(updates) == 1 else None
        if selected is not None:
            return [(kind, {**restored, updates[0]: copy.deepcopy(source[selected])})]
        for argument in updates:
            owner, slot = identified_slot(document, kind, address.get("id"), field_name(kind, argument, document))
            if owner is not None:
                restored[argument] = copy.deepcopy(owner[slot])
                continue
            key = key_of(source, field_name(kind, argument, document))
            if key is not None:
                restored[argument] = copy.deepcopy(source[key])
                continue
            owner, slot = target_slot(document, kind, address, field_name(kind, argument, document), arguments[argument])
            if owner is None:
                owner, slot = identified_slot(document, kind, address.get("id"), field_name(kind, argument, document))
            if owner is None:
                owner, slot = qualified_slot(document, kind, field_name(kind, argument, document), arguments[argument])
            if owner is None:
                record, _ = optional_record(document, kind, field_name(kind, argument, document))
                if record is None:
                    return []
                restored[argument] = type(arguments[argument])() if isinstance(arguments[argument], str) else None
                continue
            restored[argument] = copy.deepcopy(owner[slot])
        return [(kind, restored)]

    if verb == "clear":
        key = key_of(document, noun)
        partner = declared_partner(kinds, ("change", "set"), noun)
        if key is None or document[key] is None or partner is None:
            return []
        return [(partner, {"new" + "".join(part.capitalize() for part in noun.split("-")): copy.deepcopy(document[key])})]

    if verb == "insert":
        items = document.get(collection_key(document, kind) or "")
        if not isinstance(items, list):
            return []
        return [("remove-" + noun, {"index": min(arguments.get("index", len(items)), len(items))})]

    if verb == "remove" and ("insert-" + noun) in kinds and "index" in arguments:
        items = document.get(collection_key(document, kind) or "")
        index = arguments.get("index")
        if not isinstance(items, list) or not isinstance(index, int) or index >= len(items):
            return []
        return [("insert-" + noun, {"index": index, noun: copy.deepcopy(items[index])})]

    if verb == "reorder":
        items = document.get(collection_key(document, kind) or "")
        source_name, target_name = positions(arguments)
        source, target = arguments.get(source_name), arguments.get(target_name)
        if not isinstance(items, list) or not isinstance(source, int) or not isinstance(target, int) or source >= len(items) or target >= len(items):
            return []
        return [(original, {source_name: min(target, len(items) - 1), target_name: source})]

    if verb in ADDS:
        collection, member = owned_collection(document, kind, arguments)
        partner = declared_partner(kinds, ("delete", "remove", "insert", "retire"), noun)
        if partner is None or collection is None:
            return []
        carrier = next((name for name in arguments if normalised(name) in (normalised(member), normalised(noun))), None)
        element = arguments.get(carrier)
        return [(partner, undo_address(collection, element, {name: value for name, value in arguments.items() if name != carrier}, member))]

    if verb in DROPS:
        collection, member = owned_collection(document, kind, arguments)
        if collection is None:
            return []
        container, member_key = member_slot(collection, {name: value for name, value in arguments.items() if name != "index" or isinstance(collection, list)})
        if container is None:
            return []
        captured = copy.deepcopy(container[member_key])
        owner_address = {name: value for name, value in arguments.items() if name not in ("index", "id") and normalised(name) != normalised(member) and not normalised(name).endswith(normalised(member) + "id")}
        partner = declared_partner(kinds, ("create", "add", "insert", "introduce"), noun)
        if partner is not None:
            payload = dict(owner_address)
            payload[member] = captured
            if isinstance(container, list):
                payload["index"] = member_key
            return [(partner, payload)]
        setter = declared_partner(kinds, ("change", "set", "replace"), noun)
        if setter is None:
            return []
        return [(setter, dict(arguments, **{"new_value": captured}))]

    return []


def declared_partner(kinds, verbs, noun):
    """🤝 The inverse partner the verb table names, restricted to the kinds this vocabulary actually
    declares — "a real inverse partner verb in the SAME dispatch enum"."""
    return next(("%s-%s" % (verb, noun) for verb in verbs if "%s-%s" % (verb, noun) in kinds), None)


def undo_address(collection, element, arguments, noun):
    """🪪 The address that undoes a `create`/`add`: the element's own native key where it has one, its
    landing position where the collection is an anonymous ordered list, and the forward address of the
    owner it was added to in either case."""
    address = {name: value for name, value in arguments.items() if normalised(name) != normalised(noun) and name != "index"}
    identity = element.get("id") if isinstance(element, dict) else None
    if identity is None and isinstance(element, dict) and isinstance(element.get("identity"), dict):
        identity = next((value for key, value in element["identity"].items() if "article" in key or key == "id"), None)
    if identity is not None:
        address["%s_id" % noun.replace("-", "_") if address else "id"] = identity
        return address
    if isinstance(collection, list):
        address["index"] = len(collection) if arguments.get("index") is None else min(arguments["index"], len(collection))
    return address
# endregion 🔖️Verbs
