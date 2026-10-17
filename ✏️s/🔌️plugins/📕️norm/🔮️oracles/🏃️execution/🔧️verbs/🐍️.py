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


def apply_mutation(base, kind, arguments, view=MISSING):
    """⚙️ Applies one typed mutation and returns the resulting document, raising [`Refused`] when the
    verb's addressing law cannot be satisfied — an out-of-range index, a field the document has no key
    for, or a composed child slot whose new identity is not derivable from the document alone. Any
    derived mirror the document keeps is rebuilt on the way out, never left stale."""
    return rebuild_derived(derived_view(base) if view is MISSING else view, apply_verb(copy.deepcopy(base), kind, arguments))


def apply_verb(document, kind, arguments):
    """⚙️ The verb itself, on a document this call owns — inside the record a nested addressing selects."""
    steps, record = descent(document, kind, arguments)
    if record is not None:
        apply_verb(record, nested_kind(kind, steps), nested_arguments(arguments, steps))
        return document
    if any(isinstance(value, int) for _, value in steps):
        raise Refused("%s addresses a nested record %s that this document does not hold" % (kind, json.dumps(dict(steps))))
    verb = kind.split("-")[0]
    updates = new_value_arguments(arguments)
    address = {key: value for key, value in arguments.items() if key not in updates}

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
            items = indexed_collection(document, kind, [named_field(argument) for argument in updates])
            index = address["index"]
            if not isinstance(items, list) or not isinstance(index, int) or index >= len(items):
                raise Refused("%s addresses element %s of a collection that does not hold it" % (kind, index))
            for argument in updates:
                owner, key = qualified_field(items[index], kind, named_field(argument))
                if owner is None:
                    raise Refused("%s names element field %r, which this document has no key for" % (kind, named_field(argument)))
                owner[key] = copy.deepcopy(arguments[argument])
            return document
        selected = selected_field(document, kind, address) if len(updates) == 1 else None
        if selected is not None:
            document[selected] = copy.deepcopy(arguments[updates[0]])
            return document
        for argument in updates:
            key = key_of(document, named_field(argument))
            if key is None:
                owner, slot = target_slot(document, kind, address, named_field(argument), arguments[argument])
                if owner is None:
                    owner, slot = qualified_slot(document, kind, named_field(argument), arguments[argument])
                if owner is None:
                    raise Refused("%s names %r, which this document has no key for at its root or under the owner the kind's noun locates" % (kind, named_field(argument)))
                owner[slot] = copy.deepcopy(arguments[argument])
                continue
            if is_composed_slot(document[key]) and not is_composed_slot(arguments[argument]):
                raise Refused("%s would write content into the composed child slot %r, whose childId is content-addressed by a function no specification in this repository states" % (kind, key))
            document[key] = copy.deepcopy(arguments[argument])
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
    steps, record = descent(document, kind, arguments)
    if record is not None:
        inner = nested_kind(kind, steps)
        prefix = kind.split("-", 1)[1][: len(kind.split("-", 1)[1]) - len(inner.split("-", 1)[1])]
        outer = lambda step: next((candidate for candidate in (step.replace("-", "-" + prefix, 1), step) if candidate in kinds), step)
        return [(outer(step), {**dict(steps), **payload}) for step, payload in inverse_mutation([nested_kind(candidate, steps) for candidate in kinds], record, inner, nested_arguments(arguments, steps))]
    if any(isinstance(value, int) for _, value in steps):
        return []
    verb = kind.split("-")[0]
    updates = new_value_arguments(arguments)
    address = {key: value for key, value in arguments.items() if key not in updates}
    noun = "-".join(kind.split("-")[1:])

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
            items = indexed_collection(document, kind, [named_field(argument) for argument in updates])
            index = address["index"]
            if not isinstance(items, list) or not isinstance(index, int) or index >= len(items):
                return []
            restored = dict(address)
            for argument in updates:
                owner, key = qualified_field(items[index], kind, named_field(argument))
                if owner is None:
                    return []
                restored[argument] = copy.deepcopy(owner[key])
            return [(kind, restored)]
        restored = dict(address)
        selected = selected_field(source, kind, address) if len(updates) == 1 else None
        if selected is not None:
            return [(kind, {**restored, updates[0]: copy.deepcopy(source[selected])})]
        for argument in updates:
            key = key_of(source, named_field(argument))
            if key is not None:
                restored[argument] = copy.deepcopy(source[key])
                continue
            owner, slot = target_slot(document, kind, address, named_field(argument), arguments[argument])
            if owner is None:
                owner, slot = qualified_slot(document, kind, named_field(argument), arguments[argument])
            if owner is None:
                return []
            restored[argument] = copy.deepcopy(owner[slot])
        return [(kind, restored)]

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
        return [(kind, {source_name: min(target, len(items) - 1), target_name: source})]

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
        owner_address = {name: value for name, value in arguments.items() if name != "index" and normalised(name) != normalised(member) and not normalised(name).endswith(normalised(member) + "id")}
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
