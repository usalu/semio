"""📄️ Document access: the wire spelling helpers, the document model, its entity tables, the derived views and the refusal signal."""

from __future__ import annotations

# region 🔖️Imports
import copy
import itertools
import json
import re
# endregion 🔖️Imports


# region 🔖️Wire
def normalised(name):
    """🔤 The spelling-insensitive identity of a field name. The naming mechanic is stated in words
    (`new_<field>`), and the committed payloads write it snake_case in some subsets and camelCase in
    others, so every lookup compares normalised forms instead of a transcribed table."""
    return re.sub(r"[-_ ]", "", str(name)).lower()


def unwrap(wire):
    """📨 Splits a committed mutation document into its kind tag and its argument object, accepting
    both serde taggings the committed vectors use — internally tagged and externally tagged."""
    if isinstance(wire, dict) and isinstance(wire.get("mutation"), str):
        return wire["mutation"], {key: value for key, value in wire.items() if key != "mutation"}
    if isinstance(wire, dict) and len(wire) == 1:
        tag = next(iter(wire))
        if isinstance(wire[tag], dict):
            return tag, wire[tag]
    raise AssertionError("unrecognised mutation wire form: %s" % json.dumps(wire))


def new_value_arguments(arguments):
    """🆕 The arguments carrying NEW values; every other argument addresses a target."""
    return [key for key in arguments if normalised(key).startswith("new")]


def named_field(argument):
    """🎯 The field a `new_<field>` argument names, with the `new` prefix removed."""
    return re.sub(r"^new[-_]?", "", str(argument), flags=re.IGNORECASE)


def key_of(document, name):
    """🔎 The document key whose normalised spelling is `name`'s, or `None`."""
    want = normalised(name)
    return next((key for key in document if normalised(key) == want), None)
# endregion 🔖️Wire


# region 🔖️Document
def is_composed_slot(value):
    """🧩 A composed CHILD slot holds a handle — a `childId` plus the target `ArtifactRef` — never the
    child's content. The `childId` is content-addressed by a function no document in this repository
    specifies, so an implementation holding only the document cannot mint a new one."""
    return isinstance(value, dict) and "childId" in value and "target" in value


def collection_key(document, kind):
    """📚 The collection an `insert`/`remove`/`reorder` kind addresses, found from the verb's noun the
    way the derivation rules pair `insert-<singular>` with the `<plural>` field. A document whose only
    collection is a composed child slot resolves to that slot, so the refusal below can say WHY rather
    than merely that no field was found."""
    noun = "-".join(kind.split("-")[1:])
    for candidate in (noun, noun + "s", re.sub(r"s$", "", noun)):
        key = key_of(document, candidate)
        if key is not None and isinstance(document.get(key), list):
            return key
    for candidate in (noun, noun + "s", re.sub(r"s$", "", noun)):
        key = key_of(document, candidate)
        if key is not None:
            return key
    segments = noun.split("-")
    for cut in range(len(segments) - 1, 0, -1):
        for candidate in collection_names("-".join(segments[:cut])):
            key = key_of(document, candidate)
            if key is not None and isinstance(document.get(key), list):
                return key
    for cut in range(1, len(segments)):
        for candidate in collection_names("-".join(segments[cut:])):
            key = key_of(document, candidate)
            if key is not None and isinstance(document.get(key), list):
                return key
    lists = [key for key, value in document.items() if isinstance(value, list)]
    if len(lists) == 1:
        return lists[0]
    slots = [key for key, value in document.items() if is_composed_slot(value)]
    return slots[0] if len(slots) == 1 else None


def collection_names(entity):
    """🔠 The spellings a collection of `entity` records may carry — the regular `-s` plural first, then
    the `-y → -ies` one, then the bare noun — so `storey` finds `storeys` and `assessment` `assessments`."""
    return (entity + "s", re.sub(r"y$", "ies", entity), entity)


def facet_slot(document, name, value):
    """🧱 The slot an `update` facet argument replaces as a whole: the document field its name spells,
    or the member of the like-named collection that carries the facet's own `id` — `bridgeFatigueItem`
    naming one item of `bridgeFatigue` — else, an upsert, the slot one past that collection's end."""
    key = key_of(document, name)
    if key is not None:
        return document, key
    entity = facet_entity(name)
    for candidate in collection_names(entity):
        parent, key = find_container(document, candidate)
        if parent is not None and isinstance(parent[key], list) and isinstance(value, dict) and "id" in value:
            position = next((position for position, member in enumerate(parent[key]) if isinstance(member, dict) and member.get("id") == value["id"]), None)
            return parent[key], len(parent[key]) if position is None else position
    return None, None


def facet_entity(name):
    """🏷️ The entity an `update` facet argument names: its own spelling, less a trailing `Item`."""
    return re.sub(r"(?<=[a-z0-9])[-_]?[Ii]tem$", "", str(name))


def place(owner, slot, value):
    """📥 Writes one slot, appending when the slot is one past the end of a list."""
    if isinstance(owner, list) and slot == len(owner):
        owner.append(copy.deepcopy(value))
    else:
        owner[slot] = copy.deepcopy(value)


def kebab(name):
    """🔤 A camelCase or snake_case name in the kind noun's kebab spelling."""
    return re.sub(r"(?<!^)(?=[A-Z])", "-", str(name)).lower().replace("_", "-")


def qualified_field(member, kind, name):
    """🔬 The `(owner, key)` of `name` inside one collection member, preferring the sub-record the kind's noun
    names just before the field — `change-beam-stud-spacing-m {index, newSpacingM}` writes the beam's
    `studs.spacingM`, not the beam's own `spacingM` — else the shallowest field so spelled."""
    segments, field = kind.split("-")[1:], kebab(name).split("-")
    if len(segments) > len(field) and segments[-len(field):] == field:
        rest = segments[: -len(field)]
        for cut in range(len(rest)):
            for candidate in collection_names("-".join(rest[cut:])):
                holder = key_of(member, candidate) if isinstance(member, dict) else None
                if holder is not None and isinstance(member[holder], dict) and key_of(member[holder], name) is not None:
                    return member[holder], key_of(member[holder], name)
    return member_field(member, name)


def selected_field(document, kind, address):
    """🎚️ The field a selector argument picks out of a family: `change-buckling-length {axis: "z", newLength}`
    writes `bucklingLengthZ` — the kind's noun followed by the selector's value — where the record holds it."""
    if not isinstance(document, dict):
        return None
    noun = kind.split("-", 1)[1]
    return next((key_of(document, noun + "-" + value) for value in address.values() if isinstance(value, str) and key_of(document, noun + "-" + value) is not None), None)


def qualified_slot(document, kind, name, replacement):
    """🧷 The last-resort slot of a `new<Field>` no owner locates: the field the kind's noun followed by the
    field spells (`change-design-working-life {newYears}` writes `designWorkingLifeYears`), else the shallowest
    field a trailing part of the field's own name spells that could hold the value
    (`change-manufacturer-file {newManufacturerFile}` writes the catalog's `file`)."""
    key = key_of(document, kind.split("-", 1)[1] + "-" + str(name))
    if key is not None:
        return document, key
    key = unit_field(document, kind.split("-", 1)[1])
    if key is not None and compatible(document[key], replacement):
        return document, key
    segments = kebab(name).split("-")
    for cut in range(1, len(segments)):
        parent, key = find_container(document, "-".join(segments[cut:]))
        if parent is not None and compatible(parent[key], replacement):
            return parent, key
    return None, None


#: 📏 The unit spellings a norm snapshot field appends to the quantity it names — metres, square and cubic
#: metres, newtons and kilonewtons, newton- and kilonewton-metres, newtons per metre, per metre and per square
#: metre, pascals, seconds, kilograms and degrees.
UNITS = ("m", "m2", "m3", "mm", "n", "kn", "nm", "knm", "n-per-m", "per-m", "per-m2", "pa", "kpa", "mpa", "s", "kg", "deg")


def unit_field(document, noun):
    """📏 The field that spells `noun` followed by a unit of measure — the quantity-then-unit naming the norm
    snapshots use (`hM` for `h`, `mCritNm` for `m-crit`, `qLineNPerM` for `q-line`) — whole, or as the trailing
    words of a longer name (`steelPlateThicknessM` for `plate-thickness`)."""
    if not isinstance(document, dict):
        return None
    for key in document:
        if any(normalised(key) == normalised(noun + unit) or kebab(key).endswith("-" + noun + "-" + unit) for unit in UNITS):
            return key
    return None


def member_field(member, name):
    """🔬 The `(owner, key)` of the shallowest field spelled `name` inside one collection member — its own
    fields first, then its nested records and lists breadth-first in order — or `(None, None)`. The naming
    mechanic names a field, not its depth: `change-accidental-assumed-force` writes the `assumedForce` the
    case's impact record carries."""
    frontier = [member]
    while frontier:
        nested = []
        for node in frontier:
            if isinstance(node, dict):
                key = key_of(node, name)
                if key is not None:
                    return node, key
                nested.extend(value for value in node.values() if isinstance(value, (dict, list)))
            elif isinstance(node, list):
                nested.extend(value for value in node if isinstance(value, (dict, list)))
        frontier = nested
    return None, None


def indexed_collection(document, kind, names):
    """📚 The ordered collection an index-addressed setter writes: the one the kind's noun names, else the one
    collection whose every member carries each named field — `change-self-weight-assumed-gk` finds the
    collection whose members hold `assumedGk`, where no spelling of `self-weight` names it."""
    items = document.get(collection_key(document, kind) or "")
    if isinstance(items, list):
        return items
    carriers = [value for value in document.values() if isinstance(value, list) and value and all(member_field(member, name)[0] is not None for member in value for name in names)]
    return carriers[0] if len(carriers) == 1 else None


#: 🔀️ The normalised spellings of `reorder`'s two positional arguments; positions, never addressing steps.
POSITIONS = {"from": "to", "fromindex": "toindex"}
POSITIONS = {**POSITIONS, **{value: key for key, value in POSITIONS.items()}}


def positions(arguments):
    """🔀️ The argument names `reorder` carries its source and target positions under: `from`/`to` or
    `fromIndex`/`toIndex`, in whichever spelling the wire writes."""
    source = next((key for key in arguments if normalised(key) in ("from", "fromindex")), "from")
    target = next((key for key in arguments if normalised(key) == POSITIONS.get(normalised(source), "to")), "to")
    return source, target


def descent(document, kind, arguments):
    """🪜 The addressing steps and the record they select: the `<entity>Index`/`<entity>Id` steps in wire order,
    else in the first other order that selects a record — the nesting a document holds is the one order its
    records resolve in, so `change-member-load-duration {actionId, memberId}` descends into the member before
    its action — else led by a bare `index` into the collection the kind's leading noun names —
    `change-beam-action-q-area-pa {index, actionIndex, newQAreaPa}` writes action `actionIndex` of beam `index`."""
    steps = address_steps(kind, arguments)
    record = addressed_record(document, steps) if steps else None
    if record is None and steps:
        for order in list(itertools.permutations(range(len(address_steps(kind, arguments, order=())))))[1:]:
            permuted = address_steps(kind, arguments, order)
            found = addressed_record(document, permuted) if permuted else None
            if found is not None:
                return permuted, found
    if record is None and steps and isinstance(arguments.get("index"), int) and not isinstance(arguments.get("index"), bool):
        led = [("index", arguments["index"])] + steps
        found = addressed_record(document, led, kind)
        if found is not None:
            return led, found
    return steps, record


def address_steps(kind, arguments, order=None):
    """🪜 The addressing arguments that descend to the record a nested kind acts inside: every
    `<entity>Index` position and `<entity>Id` native key, in wire order or in the given `order` of their wire
    positions (`()` asks for every one of them, untrimmed), never a new value. The last `<entity>Id` of a
    `remove`/`delete` without a bare `index` names the removed member itself, so it stays with the verb
    instead of descending into the record it removes."""
    steps = [(key, value) for key, value in arguments.items() if not normalised(key).startswith("new") and normalised(key) not in POSITIONS and ((re.search(r"[a-z0-9](Index|_index)$", key) and isinstance(value, int)) or (re.search(r"[a-z0-9](Id|_id)$", key) and isinstance(value, str)))]
    if order == ():
        return steps
    if order is not None:
        steps = [steps[at] for at in order]
    if steps and kind.split("-")[0] in ("remove", "delete") and "index" not in arguments and re.search(r"(Id|_id)$", steps[-1][0]):
        steps = steps[:-1]
    return steps


def addressed_record(document, steps, kind=None):
    """🧭 The record a nested addressing selects, or `None`: each step picks, inside the record the previous
    one selected, the member at an `<entity>Index` position or the member carrying an `<entity>Id` as its
    native `id`, of the `<entity>` collection found by spelling — else, for a native key, of the one list
    that holds it (`bridgeId` finds `thermalBridges`, `ventId` finds `ventSystems`). A bare `index` step
    names the collection of the kind's leading noun."""
    node = document
    for argument, value in steps:
        if not isinstance(node, dict):
            return None
        entity = kind.split("-")[1] if argument == "index" and kind else re.sub(r"[-_]?(index|id)$", "", argument, flags=re.IGNORECASE)
        named = [node[key_of(node, name)] for name in collection_names(entity) if key_of(node, name) is not None and isinstance(node[key_of(node, name)], list)]
        holders = [items for items in node.values() if isinstance(items, list) and any(member_matches(member, value) for member in items)] if isinstance(value, str) else []
        items = named[0] if named else (holders[0] if len(holders) == 1 else None)
        if items is None:
            return None
        if isinstance(value, int):
            node = items[value] if value < len(items) else None
        else:
            node = next((member for member in items if member_matches(member, value)), None)
    return node if isinstance(node, dict) else None


def nested_arguments(arguments, steps):
    """✂️ The arguments left for the verb once the addressing steps have descended to its record."""
    descended = {key for key, _ in steps}
    return {key: value for key, value in arguments.items() if key not in descended}


def nested_kind(kind, steps):
    """🏷️ The kind as the selected record reads it: every entity a step descended through is dropped from the
    front of the noun, so `change-member-label-en {memberId, newValue}` writes the member's own `labelEn`."""
    verb, noun = kind.split("-", 1)
    for argument, _ in steps:
        entity = noun.split("-")[0] if argument == "index" else re.sub(r"(?<!^)(?=[A-Z])", "-", re.sub(r"[-_]?(index|id)$", "", argument, flags=re.IGNORECASE)).lower().replace("_", "-")
        if noun.startswith(entity + "-"):
            noun = noun[len(entity) + 1:]
    return f"{verb}-{noun}"
# endregion 🔖️Document


# region 🔖️Entities
def singular(noun):
    """1️⃣ The singular of a collection noun."""
    return re.sub(r"ies$", "y", noun) if noun.endswith("ies") else re.sub(r"s$", "", noun)


def plural(noun):
    """*️⃣ The plural of an entity noun — `-ies`, `-es` after a sibilant (`product-classes`,
    `product-indexes`), else `-s`."""
    return noun[:-1] + "ies" if noun.endswith("y") else noun + "es" if re.search(r"(s|x|z|ch|sh)$", noun) else noun + "s"


def find_container(document, name):
    """🧭 The shallowest object in the document tree whose key spells `name`. Derivation rule 2 says a
    vocabulary is derived from the SNAPSHOT SHAPE, so the shape is where the container is looked up —
    `create-subject` finds `dictionary.subjects`, `create-product-group` finds
    `catalogue.product_groups` — rather than from a path transcribed into this file."""
    want, frontier = normalised(name), [document]
    while frontier:
        nested = []
        for node in frontier:
            if not isinstance(node, dict):
                continue
            for key, value in node.items():
                if normalised(key) == want:
                    return node, key
            nested.extend(value for value in node.values() if isinstance(value, dict))
        frontier = nested
    return None, None


def member_matches(member, identity):
    """🪪 Whether a collection member carries `identity` as its native key. The addressing convention
    names the format's own key, which is a bare `id` in some of these documents and a nested identity
    record — a manufacturer code plus an article number — in others."""
    if not isinstance(member, dict):
        return False
    if member.get("id") == identity:
        return True
    nested = member.get("identity")
    return isinstance(nested, dict) and identity in nested.values()


def duplicated(collection, element):
    """⛔ Whether bringing `element` in would repeat a native `id` the collection already holds — the
    `mutation.duplicate-id` refusal of every verb that adds a member."""
    if not isinstance(element, dict) or "id" not in element:
        return False
    if isinstance(collection, dict):
        return element["id"] in collection
    return isinstance(collection, list) and any(isinstance(member, dict) and member.get("id") == element["id"] for member in collection)


def member_slot(container, address):
    """🎯 The `(owner, key)` slot one address selects inside a container, so the caller can read it,
    replace it or delete it. A map is addressed by its key, a list by `index` or by native identity."""
    values = [value for key, value in address.items() if key != "index"]
    if isinstance(container, dict):
        for value in values:
            for key in container:
                if str(key) == str(value):
                    return container, key
        return None, None
    if isinstance(container, list):
        if "index" in address and isinstance(address["index"], int) and address["index"] < len(container):
            return container, address["index"]
        for value in values:
            for position, member in enumerate(container):
                if member_matches(member, value):
                    return container, position
    return None, None


def identity_slot(entity, name):
    """🏷️ The slot `rename` writes. The verb table calls it "the identity field (`name`/`key`/`code`)";
    in these documents it is either a plain field of that name or the localised name record's
    `preferred.text`, which may sit one level down in the entity's `metadata`."""
    if isinstance(entity, dict):
        key = key_of(entity, name)
        if key is not None:
            return entity, key
        for holder in (entity, entity.get("metadata")):
            if isinstance(holder, dict) and isinstance(holder.get("names"), dict) and isinstance(holder["names"].get("preferred"), dict):
                return holder["names"]["preferred"], "text"
    return None, None


def compatible(current, replacement):
    """🧮 Whether a new value could be the one that belongs in a slot. `change-<member>{key, new_value}`
    is ambiguous when the member itself carries a `value` field: the verb table says the argument sets
    ONE field, so the argument's JSON type decides which of the two slots it can be."""
    if isinstance(current, bool) != isinstance(replacement, bool):
        return False
    for shape in (dict, list, str):
        if isinstance(current, shape) != isinstance(replacement, shape):
            return False
    return True


def target_slot(document, kind, address, name, replacement=None):
    """📌 The slot a rule-2 setter writes: the noun locates an owner in the snapshot shape, the address
    selects a member of it, and `name` — the `new_<field>` argument's field, or the noun's own trailing
    segments — selects the field inside. Returns `(owner, key)` or `(None, None)`."""
    segments = kind.split("-")[1:]
    for finder in (find_container, suffixed_container):
        for cut in range(len(segments), 0, -1):
            owner_name = "-".join(segments[:cut])
            parent, key = finder(document, plural(owner_name))
            if parent is None:
                parent, key = finder(document, owner_name)
            if parent is None or (finder is suffixed_container and not isinstance(parent[key], (list, dict))):
                continue
            owner = parent[key]
            if address:
                container, member_key = member_slot(owner, address)
                if container is None:
                    values = [value for name, value in address.items() if name != "index"]
                    if isinstance(owner, dict) and len(values) == 1:
                        return owner, str(values[0])
                    continue
                member = container[member_key]
                if name is not None and isinstance(member, dict):
                    inner = key_of(member, name)
                    if inner is not None:
                        if replacement is None or compatible(member[inner], replacement):
                            return member, inner
                    else:
                        holder, field = identity_slot(member, name)
                        if holder is not None:
                            return holder, field
                        rest = "-".join(segments[cut:])
                        if rest and key_of(member, rest) is not None:
                            return member, key_of(member, rest)
                return container, member_key
            if name is not None and isinstance(owner, dict):
                slot = (owner, key_of(owner, name)) if key_of(owner, name) is not None else identity_slot(owner, name)
                if slot[0] is not None:
                    return slot
            if cut == len(segments):
                return parent, key
    return None, None


def suffixed_container(document, name):
    """🧭 The shallowest object whose key ENDS with `name`'s words, on a word boundary, and is longer than
    it — the owner a kind names by its head noun alone: `change-wall-base-width` finds `retainingWalls`,
    while `bridgeNObs` is no spelling of `bs`."""
    want, frontier = kebab(name), [document]
    while frontier:
        nested = []
        for node in frontier:
            if not isinstance(node, dict):
                continue
            for key in node:
                if kebab(key).endswith("-" + want):
                    return node, key
            nested.extend(value for value in node.values() if isinstance(value, dict))
        frontier = nested
    return None, None
# endregion 🔖️Entities


# region 🔖️DerivedViews
def walk(node, path=()):
    """🚶 Every `(path, value)` in a document tree."""
    yield path, node
    if isinstance(node, dict):
        for key, value in node.items():
            yield from walk(value, path + (key,))
    elif isinstance(node, list):
        for position, value in enumerate(node):
            yield from walk(value, path + (position,))


def at(node, path):
    """📍 The value one path selects, or `None`."""
    for step in path:
        try:
            node = node[step]
        except (KeyError, IndexError, TypeError):
            return None
    return node


def projection_recipe(source, entry):
    """🧾 How one derived entry is projected from its source record: a path per scalar field, and for a
    list field the path to a list plus the member key it collects. Read off the document itself rather
    than declared here, and only trusted once `derived_entries` has reproduced the committed view."""
    recipe = {}
    for field, value in entry.items():
        if isinstance(value, list):
            recipe[field] = next((("collect", path, key) for path, node in walk(source) if isinstance(node, list) and len(node) == len(value)
                                  for key in (node[0].keys() if node and isinstance(node[0], dict) else ())
                                  if [member.get(key) for member in node] == value), None)
        else:
            recipe[field] = next((("read", path) for path, node in walk(source) if node == value and not isinstance(node, (dict, list))), None)
        if recipe[field] is None:
            return None
    return recipe


def derived_entries(recipe, sources):
    """🪞 Applies a recipe to every source record."""
    entries = []
    for source in sources:
        entry = {}
        for field, step in recipe.items():
            if step[0] == "collect":
                members = at(source, step[1])
                if not isinstance(members, list):
                    return None
                entry[field] = [member.get(step[2]) for member in members]
            else:
                entry[field] = copy.deepcopy(at(source, step[1]))
        entries.append(entry)
    return entries


def derived_view(base):
    """🔎 The `(view_path, source_path)` pair of a derived mirror this document keeps — a list of entries
    that is wholly a projection of another list, and must therefore be rebuilt whenever that other list
    moves. Discovered from the base document, so nothing here is declared for a subset that has none. A
    list nested inside the records it would project (or containing them) is part of those records, never
    a mirror of them: a one-element `buildings[0].storeys[0].variables` reads back from `buildings` too."""
    for path, node in walk(base):
        if not isinstance(node, list) or not node or not all(isinstance(member, dict) for member in node):
            continue
        for other, records in walk(base):
            if path[:len(other)] == other or other[:len(path)] == path or not isinstance(records, list) or len(records) != len(node) or not all(isinstance(record, dict) for record in records):
                continue
            recipes = [projection_recipe(record, entry) for record, entry in zip(records, node)]
            if recipes and all(recipe is not None for recipe in recipes) and len({tuple(sorted(recipe.items(), key=str)) for recipe in recipes}) == 1:
                if derived_entries(recipes[0], records) == node:
                    return path, other, recipes[0]
    return None


MISSING = object()


def rebuild_derived(found, document):
    """🔁 Rebuilds a derived mirror after the collection it projects has moved. Refuses rather than
    guesses when the recipe read off the base cannot project a record."""
    if found is None:
        return document
    view_path, source_path, recipe = found
    records = at(document, source_path)
    if not isinstance(records, list):
        return document
    entries = derived_entries(recipe, records)
    if entries is None:
        raise Refused("a derived view of %s could not be projected from the mutated records" % ".".join(str(step) for step in source_path))
    at(document, view_path[:-1])[view_path[-1]] = entries
    return document
# endregion 🔖️DerivedViews


# region 🔖️Refusal
class Refused(Exception):
    """🚫 A mutation this vocabulary cannot express on this document; the document must not move. `code` is
    the frozen outcome code the refusal means, where the verb table fixes one."""

    def __init__(self, reason, code=None):
        super().__init__(reason)
        self.code = code
# endregion 🔖️Refusal
