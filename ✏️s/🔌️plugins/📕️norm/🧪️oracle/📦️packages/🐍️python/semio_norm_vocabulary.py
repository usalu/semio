"""🐍️ The ONE independent Python implementation of the norm mutation vocabulary — the second producer
the fifteen `📕️norm` differential comparisons need, written once and imported by all fifteen.

WHY THIS IS ONE MODULE AND NOT FIFTEEN. Every `s.norm.*` subset is a semio-NATIVE artifact and no
third-party library, in any ecosystem, reads or writes one. That was checked rather than assumed:
PyPI serves no `en1990`, `din18599`, `vdi3805` or `iso16757` distribution at all, and the nearest
real packages (`structuralcodes`, `concreteproperties`, `anastruct`) implement design-code FORMULAE
and speak no interchange format, so none of them could judge a `<Standard>Mutation` vocabulary. The
reference is therefore a second IMPLEMENTATION — and it is a second implementation of ONE thing,
because what the fifteen subsets share is not a family resemblance but a single generative document:

* `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️taxonomy.md` — the closed
  verb table (`change` sets one scalar field and its inverse is `change` with the old value; `update`
  sets one cohesive multi-field facet and its inverse is `update` with the old facet; `insert` takes a
  FINAL-state index and `remove` a BASE-state one, and they invert each other; `reorder{from,to}`
  inverts to `reorder{from: min(to, len-1), to: from}`), the naming mechanics — "New-value fields are
  `new_<field>`; address fields are bare" — and the addressing convention, "Inverse always computed
  from `base`" and "Missing target ⇒ `inverse` returns `Vec::new()`".
* the same ticket's `📓️derivation-rules.md` — the shape rules that decide which verbs a document-root
  scalar, an index-keyed ordered collection and a composed child slot each earn.

Those two documents say the same thing to all fifteen subsets. Writing them out fifteen times would
have produced fifteen files whose engines were byte-identical — which is exactly what the fifteen
committed adapters were before this module existed, and it made the reference surface look fifteen
times larger than the evidence it carries. **A shared bug in a copied oracle agrees with itself in
every case that shares it**, and fifteen copies hide that where one import states it. So: one engine,
one bug surface, declared. What each subset still contributes for itself is its committed kind list,
its committed specification vectors, its real committed document and its envelope token — the four
things `Subset` below carries, and the only four things that are genuinely per-standard.

Nothing here is imported from or transliterated out of the Rust it judges. The document field a
`new*` argument names is resolved by NORMALISED SPELLING against the document's own keys — which is
exactly what the naming mechanic states and is why the same code reads both the snake_case and the
camelCase payload spellings the fifteen norm subsets committed — never from a mapping table copied
out of `🧬️mutations/**`.

⚠️ Honest boundary, the CARRIER. `.dsl.semio` has no specification here: every norm subset's committed
`🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio` is the repository-wide placeholder whose whole
body is `payload = OCTET+`, and the framework's own DSL notation module publishes no grammar document
either. `identity_handler` below therefore reads the committed artifact at the CARRIER level — the
envelope preamble and the ordered `key=value` fields as written — a shape derived from those
artifacts' own committed bytes and PINNED by re-emitting each file byte for byte, which a misreading
could not do. It deliberately does not map carrier tokens onto the JSON snapshot's enum spellings:
that mapping is stated nowhere, and inferring it would be reverse-engineering rather than a second
reading. Two subsets — `iso16757` and `vdi3805` — write a nesting notation this grammar-less carrier
cannot describe, and this module REFUSES them rather than guessing; that refusal is a standing
finding about the specification, not a defect to be tuned away.

🔗 Reached by the fifteen adapters through the `oracleHostPackage` this plugin's
`🔣️oracle.json` declares, which puts this directory on the generated host's import path.
"""


from __future__ import annotations

# region 🔖️Imports
import copy
import json
import re

from semio_repo_test import Adapter, Context, Outcome, digest

# endregion 🔖️Imports


# region 🔖️Subset
class Subset:
    """📕️ Everything one norm subset contributes to this engine, and the whole of what distinguishes
    the fifteen reference adapters from one another: the closed kind list its committed catalog
    declares, the committed specification vector each kind publishes, the real committed document the
    carrier round-trip reads, and the envelope token that document's preamble must carry. No verb, no
    addressing rule and no carrier rule is per-subset — the derivation rules this engine implements
    are the same document for all fifteen, so implementing them fifteen times would be fifteen copies
    of one reading, not fifteen readings."""

    def __init__(self, standard, kinds, vectors, dsl_asset, envelope, vector_root=None):
        self.standard = standard
        self.kinds = list(kinds)
        self.vectors = dict(vectors)
        self.dsl_asset = dsl_asset
        self.envelope = envelope
        self.vector_root = vector_root or VECTOR_ROOT


#: 📂 The `asset://` prefix every specification vector hangs off; identical in all fifteen subsets
#: because the mutation triad's location is fixed by the repository taxonomy, not by the standard.
VECTOR_ROOT = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
# endregion 🔖️Subset


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
    lists = [key for key, value in document.items() if isinstance(value, list)]
    if len(lists) == 1:
        return lists[0]
    slots = [key for key, value in document.items() if is_composed_slot(value)]
    return slots[0] if len(slots) == 1 else None
# endregion 🔖️Document


# region 🔖️Entities
def singular(noun):
    """1️⃣ The singular of a collection noun."""
    return re.sub(r"ies$", "y", noun) if noun.endswith("ies") else re.sub(r"s$", "", noun)


def plural(noun):
    """*️⃣ The plural of an entity noun."""
    return noun[:-1] + "ies" if noun.endswith("y") else noun + "s"


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
    for cut in range(len(segments), 0, -1):
        owner_name = "-".join(segments[:cut])
        parent, key = find_container(document, plural(owner_name))
        if parent is None:
            parent, key = find_container(document, owner_name)
        if parent is None:
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
    moves. Discovered from the base document, so nothing here is declared for a subset that has none."""
    for path, node in walk(base):
        if not isinstance(node, list) or not node or not all(isinstance(member, dict) for member in node):
            continue
        for other, records in walk(base):
            if other == path or not isinstance(records, list) or len(records) != len(node) or not all(isinstance(record, dict) for record in records):
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


# region 🔖️Verbs
class Refused(Exception):
    """🚫 A mutation this vocabulary cannot express on this document; the document must not move."""


def apply_mutation(base, kind, arguments, view=MISSING):
    """⚙️ Applies one typed mutation and returns the resulting document, raising [`Refused`] when the
    verb's addressing law cannot be satisfied — an out-of-range index, a field the document has no key
    for, or a composed child slot whose new identity is not derivable from the document alone. Any
    derived mirror the document keeps is rebuilt on the way out, never left stale."""
    return rebuild_derived(derived_view(base) if view is MISSING else view, apply_verb(copy.deepcopy(base), kind, arguments))


def apply_verb(document, kind, arguments):
    """⚙️ The verb itself, on a document this call owns."""
    verb = kind.split("-")[0]
    updates = new_value_arguments(arguments)
    address = {key: value for key, value in arguments.items() if key not in updates}

    if verb in ("change", "set", "update", "rename", "edit", "replace", "resize"):
        if not updates:
            raise Refused("%s carries no new-value argument" % kind)
        if "index" in address:
            items = document.get(collection_key(document, kind) or "")
            index = address["index"]
            if not isinstance(items, list) or not isinstance(index, int) or index >= len(items):
                raise Refused("%s addresses element %s of a collection that does not hold it" % (kind, index))
            for argument in updates:
                key = key_of(items[index], named_field(argument))
                if key is None:
                    raise Refused("%s names element field %r, which this document has no key for" % (kind, named_field(argument)))
                items[index][key] = copy.deepcopy(arguments[argument])
            return document
        for argument in updates:
            key = key_of(document, named_field(argument))
            if key is None:
                owner, slot = target_slot(document, kind, address, named_field(argument), arguments[argument])
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
        element = next((value for name, value in arguments.items() if name != "index"), None)
        if element is None:
            raise Refused("%s carries no element to insert" % kind)
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
        source, target = arguments.get("from"), arguments.get("to")
        if not isinstance(items, list) or not isinstance(source, int) or not isinstance(target, int) or source >= len(items) or target >= len(items):
            raise Refused("%s addresses positions %s→%s of a collection that does not hold them" % (kind, source, target))
        items.insert(target, items.pop(source))
        return document

    if verb in ("create", "delete", "add"):
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
    if verb in ("create", "add"):
        element = next((value for name, value in arguments.items() if normalised(name) == normalised(noun)), None)
        if element is None:
            element = next((value for name, value in arguments.items() if isinstance(value, (dict, list))), None)
        if element is None:
            raise Refused("%s carries no element to add" % kind)
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
        raise Refused("%s addresses a member this document's %r does not hold" % (kind, noun))
    if isinstance(container, dict):
        del container[member_key]
    else:
        container.pop(member_key)
    return document


def inverse_mutation(kinds, document, kind, arguments):
    """↩️ The mutation's own inverse, always computed from the BASE document, and empty when the target
    is missing — both stated by the addressing convention."""
    verb = kind.split("-")[0]
    updates = new_value_arguments(arguments)
    address = {key: value for key, value in arguments.items() if key not in updates}
    noun = "-".join(kind.split("-")[1:])

    if verb in ("change", "set", "update", "rename", "edit", "replace", "resize"):
        source = document
        if "index" in address:
            items = document.get(collection_key(document, kind) or "")
            index = address["index"]
            if not isinstance(items, list) or not isinstance(index, int) or index >= len(items):
                return []
            source = items[index]
        restored = dict(address)
        for argument in updates:
            key = key_of(source, named_field(argument))
            if key is not None:
                restored[argument] = copy.deepcopy(source[key])
                continue
            owner, slot = target_slot(document, kind, address, named_field(argument), arguments[argument])
            if owner is None:
                return []
            restored[argument] = copy.deepcopy(owner[slot])
        return [(kind, restored)]

    if verb == "insert":
        items = document.get(collection_key(document, kind) or "")
        if not isinstance(items, list):
            return []
        return [("remove-" + noun, {"index": min(arguments.get("index", len(items)), len(items))})]

    if verb == "remove" and ("insert-" + noun) in kinds:
        items = document.get(collection_key(document, kind) or "")
        index = arguments.get("index")
        if not isinstance(items, list) or not isinstance(index, int) or index >= len(items):
            return []
        return [("insert-" + noun, {"index": index, noun: copy.deepcopy(items[index])})]

    if verb == "reorder":
        items = document.get(collection_key(document, kind) or "")
        source, target = arguments.get("from"), arguments.get("to")
        if not isinstance(items, list) or not isinstance(source, int) or not isinstance(target, int) or source >= len(items) or target >= len(items):
            return []
        return [(kind, {"from": min(target, len(items) - 1), "to": source})]

    if verb in ("create", "add"):
        collection, member = owned_collection(document, kind, arguments)
        partner = declared_partner(kinds, ("delete", "remove", "insert"), noun)
        if partner is None or collection is None:
            return []
        element = next((value for name, value in arguments.items() if normalised(name) == normalised(member)), None)
        return [(partner, undo_address(collection, element, arguments, member))]

    if verb in ("delete", "remove"):
        collection, member = owned_collection(document, kind, arguments)
        if collection is None:
            return []
        container, member_key = member_slot(collection, {name: value for name, value in arguments.items() if name != "index" or isinstance(collection, list)})
        if container is None:
            return []
        captured = copy.deepcopy(container[member_key])
        owner_address = {name: value for name, value in arguments.items() if name != "index" and normalised(name) != normalised(member) and not normalised(name).endswith(normalised(member) + "id")}
        partner = declared_partner(kinds, ("create", "add", "insert"), noun)
        if partner is not None:
            payload = dict(owner_address)
            payload[member] = captured
            if isinstance(container, list) and "index" in arguments:
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


# region 🔖️Carrier
PREAMBLE = re.compile(r"^semio\s+(\S+)\s+v(\d+)\n")
TABLE_HEAD = re.compile(r"^(\S+) \[([^\]]*)\] \{$")


def split_fields(line):
    """✂️ Splits one carrier line on unquoted whitespace."""
    fields, current, quoted = [], "", False
    for character in line:
        if character == '"':
            quoted = not quoted
            current += character
        elif character.isspace() and not quoted:
            if current:
                fields.append(current)
                current = ""
        else:
            current += character
    if current:
        fields.append(current)
    return fields


def parse_dsl(envelope, text):
    """📖 Reads the committed `.dsl.semio` artifact at the carrier level: the envelope preamble, the
    ordered `key=value` field lines, and any typed table block written `name [col:TYPE …] { rows }`."""
    header = PREAMBLE.match(text)
    if header is None:
        raise AssertionError("identity-round-trip: the committed artifact does not open with a semio text preamble")
    if header.group(1) != envelope:
        raise AssertionError("identity-round-trip: expected the envelope %r, the artifact declares %r" % (envelope, header.group(1)))
    blocks, lines, cursor = [], text[header.end():].split("\n"), 0
    while cursor < len(lines):
        line = lines[cursor]
        cursor += 1
        if line == "":
            continue
        table = TABLE_HEAD.match(line)
        if table is not None:
            columns = [column.split(":", 1) for column in split_fields(table.group(2))]
            rows = []
            while cursor < len(lines) and lines[cursor] != "}":
                rows.append(split_fields(lines[cursor]))
                cursor += 1
            if cursor >= len(lines):
                raise AssertionError("identity-round-trip: table %r is never closed" % table.group(1))
            cursor += 1
            blocks.append({"table": table.group(1), "columns": columns, "rows": rows})
            continue
        fields = []
        for token in split_fields(line):
            key, separator, value = token.partition("=")
            if separator != "=":
                raise AssertionError(
                    "identity-round-trip: this artifact's carrier cannot be read by a second implementation. %r is not a "
                    "`key=value` field: the notation nests records and tables and flattens nested records into "
                    "`key=key=value` runs with no delimiter, and this repository publishes no grammar for it — the "
                    "subset's own `🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio` is the repository-wide "
                    "`payload = OCTET+` placeholder and the framework's DSL notation module carries no grammar document "
                    "either. Reconstructing these bytes would mean inferring a grammar from one example rather than "
                    "reading a specification, so this implementation refuses instead of pretending. The mutation "
                    "vocabulary above is unaffected: it is specified, and both implementations agree on all of it. This "
                    "is a documentation gap in the subset, not a defect in either codec." % token)
            fields.append([key, value])
        blocks.append({"fields": fields})
    return {"envelope": header.group(1), "version": int(header.group(2)), "blocks": blocks}


def carrier_projection(text):
    """🧵️ The canonical carrier bytes as a comparable projection: the envelope preamble, every body line
    as written, and the digest and length of what this implementation emitted. The Rust subject builds
    the identical shape from ITS re-emission, and `digest` is the coordinator's own sha256, so the two
    languages' bytes are directly comparable."""
    preamble, _newline, body = text.partition("\n")
    body = body[:-1] if body.endswith("\n") else body
    return {
        "preamble": preamble,
        "lines": body.split("\n") if body else [],
        "dslDigest": digest(text.encode("utf-8")),
        "dslLength": len(text.encode("utf-8")),
    }


def print_dsl(document):
    """🖨️ Prints a parsed carrier document back to its canonical `.dsl.semio` bytes."""
    lines = []
    for block in document["blocks"]:
        if "fields" in block:
            lines.append(" ".join("%s=%s" % (key, value) for key, value in block["fields"]))
            continue
        lines.append("%s [%s] {" % (block["table"], " ".join(":".join(column) for column in block["columns"])))
        lines.extend("  " + " ".join(row) for row in block["rows"])
        lines.append("}")
    return "semio %s v%d\n%s\n" % (document["envelope"], document["version"], "\n".join(lines))
# endregion 🔖️Carrier


# region 🔖️Laws
def conforms(scenario, got, expected, what):
    """📐 The result must BE the committed snapshot, printed in full on disagreement."""
    if got != expected:
        raise AssertionError("%s: %s\n     got: %s\nexpected: %s" % (scenario, what, json.dumps(got, sort_keys=True), json.dumps(expected, sort_keys=True)))


def observable(kind, mutated, base):
    """👁️ An applied mutation must MOVE the document; a handler that quietly did nothing would
    otherwise report a pass having changed nothing."""
    if mutated == base:
        raise AssertionError("mutate-%s: the committed vector declares this mutation applied, yet the document did not move" % kind)


def untouched(kind, mutated, base):
    """🧊 A refused mutation must leave the document bit-identical — the stricter contract of the two."""
    if mutated != base:
        raise AssertionError("mutate-%s: the committed vector declares this mutation rejected, yet the document moved" % kind)


def restores(kind, restored, base):
    """↩️ The mutation followed by its own inverse must restore the base exactly, position included."""
    if restored != base:
        raise AssertionError("inverse-%s: undoing the mutation did not restore the before-snapshot\n     got: %s\nexpected: %s" % (kind, json.dumps(restored, sort_keys=True), json.dumps(base, sort_keys=True)))
# endregion 🔖️Laws


# region 🔖️Handlers
def read_json(ctx, uri):
    """🔣️ One committed JSON fixture, read through the plan so an undeclared URI is an error."""
    return json.loads(ctx.fixture_bytes(uri).decode("utf-8"))


def vector(subset, ctx, kind):
    """🧫️ The committed `(before, mutation, after, outcome)` quintet for one kind."""
    directory, fixture = subset.vectors[kind]
    stem = "%s/%s/🧪️tests/%s" % (subset.vector_root, directory, fixture)
    return (
        read_json(ctx, "%s/📸️snapshot/⬅️before/🔣️.json" % stem),
        read_json(ctx, "%s/🦠️mutation/🔣️.json" % stem),
        read_json(ctx, "%s/📸️snapshot/➡️after/🔣️.json" % stem),
        read_json(ctx, "%s/🎯️outcome/🔣️.json" % stem),
    )


def mutate_handler(subset, kind):
    """🎯️ Applies the kind to its committed before-snapshot and asserts in role that this
    implementation reaches the committed after-snapshot under the contract the committed outcome
    declares. The projection is the resulting document, which is what parity compares."""

    def handler(ctx):
        base, wire, expected, outcome = vector(subset, ctx, kind)
        _tag, arguments = unwrap(wire)
        refusal = None
        try:
            current = apply_mutation(base, kind, arguments)
        except Refused as reason:
            current, refusal = copy.deepcopy(base), str(reason)
        status = outcome.get("status")
        if status not in ("applied", "rejected"):
            raise AssertionError("mutate-%s: unknown committed outcome status %r" % (kind, status))
        if status == "applied" and refusal is not None:
            raise AssertionError("mutate-%s: the committed vector declares this mutation applied, yet this implementation refused it: %s" % (kind, refusal))
        if status == "rejected" and refusal is None:
            raise AssertionError("mutate-%s: the committed vector declares this mutation rejected, yet this implementation applied it" % kind)
        conforms("mutate-" + kind, current, expected, "the applied document does not match the committed after-snapshot")
        if status == "applied":
            observable(kind, current, base)
        else:
            untouched(kind, current, base)
        return Outcome(current, raw=json.dumps(current, sort_keys=True).encode("utf-8"))

    return handler


def inverse_handler(subset, kind):
    """↩️ The metamorphic inverse law in role. The projection carries BOTH the mutated and the restored
    document: projecting only the restored one would make every row project the same value and the
    differential would be vacuous."""

    def handler(ctx):
        base, wire, _expected, outcome = vector(subset, ctx, kind)
        _tag, arguments = unwrap(wire)
        try:
            mutated = apply_mutation(base, kind, arguments)
        except Refused as reason:
            if outcome.get("status") == "applied":
                raise AssertionError("inverse-%s: the forward mutation could not be applied to its own committed before-snapshot: %s" % (kind, reason))
            mutated = copy.deepcopy(base)
        steps = inverse_mutation(subset.kinds, base, kind, arguments)
        if outcome.get("status") == "applied" and not steps:
            raise AssertionError("inverse-%s: this kind changes the document, so its computed inverse must not be empty" % kind)
        restored, view = mutated, derived_view(base)
        for step, payload in steps:
            try:
                restored = apply_mutation(restored, step, payload, view)
            except Refused as reason:
                raise AssertionError("inverse-%s: an inverse step was refused: %s" % (kind, reason))
        restores(kind, restored, base)
        projection = {"mutated": mutated, "restored": restored}
        return Outcome(projection, raw=json.dumps(projection, sort_keys=True).encode("utf-8"))

    return handler


def identity_handler(subset):
    """🔁️ The subset's real committed document through the carrier it is committed in. That carrier
    is this repository's own canonical printer output, so an exact re-emission is the correct answer
    and the wave's must-differ tripwire would be backwards here; what keeps the agreement from being a
    codec agreeing with itself is that these bytes are re-emitted by a SECOND implementation, and both
    sides project the digest and the length of what they emitted."""

    def handler(ctx):
        text = ctx.fixture_bytes(subset.dsl_asset).decode("utf-8")
        document = parse_dsl(subset.envelope, text)
        reprinted = print_dsl(document)
        if reprinted != text:
            raise AssertionError("identity-round-trip: re-printing the parsed document did not reproduce the committed artifact byte for byte\n     got %d bytes\nexpected %d bytes" % (len(reprinted.encode("utf-8")), len(text.encode("utf-8"))))
        if parse_dsl(subset.envelope, reprinted) != document:
            raise AssertionError("identity-round-trip: printing the document back and reparsing it lost content")
        return Outcome(carrier_projection(reprinted), raw=reprinted.encode("utf-8"))

    return handler
# endregion 🔖️Handlers


# region 🔖️Registration
def build_adapter(subset):
    """🧭️ Registration is by FULL expanded scenario id, so this mirrors the feature's `Examples` tables
    exactly. Oracle role only: registering these handlers as subjects as well would make the reference
    its own subject and manufacture a guaranteed-green self-comparison."""
    built = Adapter("python")
    for kind in subset.kinds:
        built = built.oracle("mutate-" + kind, mutate_handler(subset, kind)).oracle("inverse-" + kind, inverse_handler(subset, kind))
    return built.oracle("identity-round-trip", identity_handler(subset))
# endregion 🔖️Registration
