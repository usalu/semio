#!/usr/bin/env python3
"""🧰️ One-off: writes h14-creation-standard.json (t6-queue row 36, right after row 34) — creation entries per (kind, standard):
a kind whose editors span several standards is offered once per standard (labelled en/de with the standard, e.g. "PDF 1.7"),
each entry created by the most general editor WITHIN its standard; the create request names the entry by its dialect (the
existing `SpaceArtifactCreationDialectV1` / dialect-coordinate grammar), `withheld` rows name their standard. Anchored on the
post-state of rows A (hold A, landed) and 34 (`H14_BASE` = a tree with row 34 applied)."""
import hashlib, json, os, pathlib, re, struct

ROOT = pathlib.Path('/Users/ueli/Documents/semio')
BASE = pathlib.Path(os.environ.get('H14_BASE', str(ROOT)))
OS = '🧰️framework/🛍️products/💻️os'
RS = f'{OS}/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🦀️.rs'
TS = f'{OS}/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts'
VECTORS = f'{OS}/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🔣️.json'
SCHEMA = f'{OS}/🔨️modules/📇️directory/🧬️schema/🔣️.json'
OS_TS = f'{OS}/🟦️.ts'
WORKER = f'{OS}/🔨️modules/🏪️store/👷️worker/🟦️.ts'
SHELL = f'{OS}/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx'
OWNER_TEST = f'{OS}/🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts'
ENVELOPE_TEST = f'{OS}/🧪️tests/🧪️backbone-envelope-io/🟦️.ts'
WORKER_SCHEMA = f'{OS}/🧬️schema/🌱️space-artifact-creation-generation-v1/🔣️.json'
WORKER_FIXTURE = f'{OS}/🧫️fixtures/📇️directory/🌱️space-artifact-creation-generation-v1.json'
WGPU = f'{OS}/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️HubConnection/🎯️targets/🧊️wgpu/🦀️.rs'
WGPU_UNIT = f'{OS}/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️HubConnection/🧪️tests/🔬️wgpu-unit/🦀️.rs'
WGPU_COLLAB = f'{OS}/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/🤝️hub-collaboration/🟦️.ts'
MCP = f'{OS}/🔨️modules/🌉️mcp/🧪️tests'
CAT = '🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs'
CAT_LAW = '🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🔬️unit/🦀️.rs'
CREATION = '🌎️hub/🗿️artifact-authority/🌱️creation/🦀️.rs'
SERVICE = '🌎️hub/🗿️artifact-authority/🌱️creation/🧑‍🏭️service-v1/🦀️.rs'
CREATION_LAW = '🌎️hub/🗿️artifact-authority/🌱️creation/🧪️tests/🔬️unit/🦀️.rs'
OPERATION = '🌎️hub/🗿️artifact-authority/🌱️creation/🧫️fixtures/📚️operation-v1/🔣️.json'
INFERENCE = '🌎️hub/💡️inference/📇️catalog/🦀️.rs'
BIN_UNIT = '🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs'
HARNESS = '🌎️hub/🤝️integration-harness/🟦️.ts'
HUB_SCRIPT = '🌎️hub/📦️packages/🦀️rust/📜️script.ts'
HUB_TESTS = '🌎️hub/🧪️tests'
H = []
GIS = '{ artifactKind: "s.gis.gismap", standard: "1", subset: "any" }'


def text(file):
    return (BASE / file).read_text()


def hunk(file, old, new, all_occurrences=False):
    count = text(file).count(old)
    assert count == 1 or (all_occurrences and count >= 1), (file, count, old[:90])
    H.append({'file': file, 'old': old, 'new': new, **({'all': True} if all_occurrences else {})})


# ── os-kernel Rust schema ─────────────────────────────────────────────────────────────────────────────────────────────
hunk(RS, r'''/// 🚫️ One kind of the generation the catalog names but does not offer, with its reason.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SpaceArtifactCreationWithheldKindV1 {
    pub kind_id: String,
    pub reason: SpaceArtifactCreationWithheldReasonV1,
}''', r'''/// 🚫️ One (kind, standard) entry of the generation the catalog names but does not offer, with its reason.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SpaceArtifactCreationWithheldKindV1 {
    pub kind_id: String,
    pub standard: String,
    pub reason: SpaceArtifactCreationWithheldReasonV1,
}''')

hunk(RS, r'''    /// 🧬️ Offered and withheld kinds are each kind-sorted and duplicate-free, disjoint, together nonempty and bounded.''', r'''    /// 🧬️ Offered and withheld entries are each sorted by (kind, standard) and duplicate-free, disjoint, together nonempty and
    /// bounded: a kind whose editors span several standards is named once per standard.''')

hunk(RS, r'''            && self.kinds.windows(2).all(|pair| pair[0].kind_id < pair[1].kind_id)
            && self.withheld.iter().all(|row| identity(&row.kind_id))
            && self.withheld.windows(2).all(|pair| pair[0].kind_id < pair[1].kind_id)
            && self.withheld.iter().all(|row| self.kinds.binary_search_by(|kind| kind.kind_id.as_str().cmp(row.kind_id.as_str())).is_err())''', r'''            && self.kinds.windows(2).all(|pair| (pair[0].kind_id.as_str(), pair[0].dialect.standard.as_str()) < (pair[1].kind_id.as_str(), pair[1].dialect.standard.as_str()))
            && self.withheld.iter().all(|row| identity(&row.kind_id) && identity(&row.standard))
            && self.withheld.windows(2).all(|pair| (pair[0].kind_id.as_str(), pair[0].standard.as_str()) < (pair[1].kind_id.as_str(), pair[1].standard.as_str()))
            && self.withheld.iter().all(|row| self.kinds.binary_search_by(|kind| (kind.kind_id.as_str(), kind.dialect.standard.as_str()).cmp(&(row.kind_id.as_str(), row.standard.as_str()))).is_err())''')

hunk(RS, r'''pub struct SpaceArtifactCreateV1 {
    pub schema: String,
    pub request_id: String,
    pub expected_catalog_generation_id: String,
    pub kind_id: String,
    pub name: String,
}

impl SpaceArtifactCreateV1 {
    /// 🛡️ Validates the schema-owned scalar bounds before catalog or storage access.
    pub fn validate(&self) -> bool {
        self.schema == "semio.hub.space-artifact-create/v1"
            && request_id(&self.request_id)
            && digest(&self.expected_catalog_generation_id)
            && identity(&self.kind_id)''', r'''pub struct SpaceArtifactCreateV1 {
    pub schema: String,
    pub request_id: String,
    pub expected_catalog_generation_id: String,
    pub kind_id: String,
    pub dialect: SpaceArtifactCreationDialectV1,
    pub name: String,
}

impl SpaceArtifactCreateV1 {
    /// 🛡️ Validates the schema-owned scalar bounds before catalog or storage access. The client names the catalog entry it
    /// chose by its kind and dialect (one entry per standard); the hub creates only that entry's own editor.
    pub fn validate(&self) -> bool {
        self.schema == "semio.hub.space-artifact-create/v1"
            && request_id(&self.request_id)
            && digest(&self.expected_catalog_generation_id)
            && identity(&self.kind_id)
            && identity(&self.dialect.artifact_kind)
            && identity(&self.dialect.standard)
            && subset(&self.dialect.subset)''')

# ── TS twin ───────────────────────────────────────────────────────────────────────────────────────────────────────────
hunk(TS, '''export type SpaceArtifactCreateV1 = Readonly<{
  schema: typeof SPACE_ARTIFACT_CREATE_SCHEMA_V1;
  requestId: string;
  expectedCatalogGenerationId: string;
  kindId: string;
  name: string;
}>;''', '''export type SpaceArtifactCreateV1 = Readonly<{
  schema: typeof SPACE_ARTIFACT_CREATE_SCHEMA_V1;
  requestId: string;
  expectedCatalogGenerationId: string;
  kindId: string;
  dialect: SpaceArtifactCreationDialectV1;
  name: string;
}>;''')

hunk(TS, '''export type SpaceArtifactCreationWithheldKindV1 = Readonly<{
  kindId: string;
  reason: SpaceArtifactCreationWithheldReasonV1;
}>;''', '''export type SpaceArtifactCreationWithheldKindV1 = Readonly<{
  kindId: string;
  standard: string;
  reason: SpaceArtifactCreationWithheldReasonV1;
}>;''')

hunk(TS, '''  if (row === null || !exactFields(row, ["kindId", "reason"])) return null;
  const kindId = identity(row.kindId),
    reason = SPACE_ARTIFACT_CREATION_WITHHELD_REASONS_V1.find((known) => known === row.reason);
  return kindId !== null && reason !== undefined ? { kindId, reason } : null;''', '''  if (row === null || !exactFields(row, ["kindId", "standard", "reason"])) return null;
  const kindId = identity(row.kindId),
    standard = identity(row.standard),
    reason = SPACE_ARTIFACT_CREATION_WITHHELD_REASONS_V1.find((known) => known === row.reason);
  return kindId !== null && standard !== null && reason !== undefined ? { kindId, standard, reason } : null;''')

hunk(TS, '''/** 📥️ Seals the only client-supplied creation fields. */
export function sealSpaceArtifactCreateV1(value: Readonly<{ requestId: string; expectedCatalogGenerationId: string; kindId: string; name: string }>): SpaceArtifactCreateV1 {
  const parsedRequestId = requestId(value.requestId),
    expectedCatalogGenerationId = digest(value.expectedCatalogGenerationId),
    parsedKindId = identity(value.kindId),
    parsedName = name(value.name);
  if (parsedRequestId === null || expectedCatalogGenerationId === null || parsedKindId === null || parsedName === null) throw new Error("space artifact creation: invalid intent");
  return { schema: SPACE_ARTIFACT_CREATE_SCHEMA_V1, requestId: parsedRequestId, expectedCatalogGenerationId, kindId: parsedKindId, name: parsedName };
}''', '''/** 🧭️ The chosen catalog entry's dialect, exactly (`subset` is an identity or `*`), or `null`. */
function creationDialect(value: unknown): SpaceArtifactCreationDialectV1 | null {
  const row = record(value);
  if (row === null || !exactFields(row, ["artifactKind", "standard", "subset"])) return null;
  const artifactKind = identity(row.artifactKind),
    standard = identity(row.standard),
    subset = subsetIdentity(row.subset);
  return artifactKind !== null && standard !== null && subset !== null ? { artifactKind, standard, subset } : null;
}

/** 📥️ Seals the only client-supplied creation fields: the chosen entry (kind and dialect — one entry per standard) and a name. */
export function sealSpaceArtifactCreateV1(value: Readonly<{ requestId: string; expectedCatalogGenerationId: string; kindId: string; dialect: SpaceArtifactCreationDialectV1; name: string }>): SpaceArtifactCreateV1 {
  const parsedRequestId = requestId(value.requestId),
    expectedCatalogGenerationId = digest(value.expectedCatalogGenerationId),
    parsedKindId = identity(value.kindId),
    dialect = creationDialect(value.dialect),
    parsedName = name(value.name);
  if (parsedRequestId === null || expectedCatalogGenerationId === null || parsedKindId === null || dialect === null || parsedName === null) throw new Error("space artifact creation: invalid intent");
  return { schema: SPACE_ARTIFACT_CREATE_SCHEMA_V1, requestId: parsedRequestId, expectedCatalogGenerationId, kindId: parsedKindId, dialect, name: parsedName };
}''')

hunk(TS, '''  if (row === null || !exactFields(row, ["schema", "requestId", "expectedCatalogGenerationId", "kindId", "name"]) || row.schema !== SPACE_ARTIFACT_CREATE_SCHEMA_V1) throw new Error("space artifact creation: invalid fields");
  const result = sealSpaceArtifactCreateV1({ requestId: String(row.requestId), expectedCatalogGenerationId: String(row.expectedCatalogGenerationId), kindId: String(row.kindId), name: String(row.name) });''', '''  if (row === null || !exactFields(row, ["schema", "requestId", "expectedCatalogGenerationId", "kindId", "dialect", "name"]) || row.schema !== SPACE_ARTIFACT_CREATE_SCHEMA_V1) throw new Error("space artifact creation: invalid fields");
  const result = sealSpaceArtifactCreateV1({ requestId: String(row.requestId), expectedCatalogGenerationId: String(row.expectedCatalogGenerationId), kindId: String(row.kindId), dialect: row.dialect as SpaceArtifactCreationDialectV1, name: String(row.name) });''')

hunk(TS, '''  if (sealed.some((kind, index) => index > 0 && sealed[index - 1]!.kindId >= kind.kindId)) throw new Error("space artifact creation catalog: invalid order");''', '''  const entry = (kindId: string, standard: string): string => `${kindId}\\u0000${standard}`;
  if (sealed.some((kind, index) => index > 0 && entry(sealed[index - 1]!.kindId, sealed[index - 1]!.dialect.standard) >= entry(kind.kindId, kind.dialect.standard))) throw new Error("space artifact creation catalog: invalid order");''')

hunk(TS, '''  if (named.some((kind, index) => (index > 0 && named[index - 1]!.kindId >= kind.kindId) || sealed.some((offered) => offered.kindId === kind.kindId)))''', '''  if (named.some((kind, index) => (index > 0 && entry(named[index - 1]!.kindId, named[index - 1]!.standard) >= entry(kind.kindId, kind.standard)) || sealed.some((offered) => offered.kindId === kind.kindId && offered.dialect.standard === kind.standard)))''')

# ── JSON Schema (directory) ───────────────────────────────────────────────────────────────────────────────────────────
hunk(SCHEMA, '''          "description": "The offered kinds, sorted by kind id. Offered and withheld kinds together name every kind an editor of the generation opens: at most the hub's trusted-catalog open-target ceiling, so a catalog is never cut off.",''', '''          "description": "The offered entries, sorted by (kind id, dialect standard): one entry per standard a kind's editors span. Offered and withheld entries together name every (kind, standard) an editor of the generation opens: at most the hub's trusted-catalog open-target ceiling, so a catalog is never cut off.",''')
hunk(SCHEMA, '''          "description": "The kinds an editor of the generation opens that are not offered, each with its reason, sorted by kind id and disjoint from `kinds`.",''', '''          "description": "The (kind, standard) entries an editor of the generation opens that are not offered, each with its reason, sorted by (kind id, standard) and disjoint from `kinds`.",''')
hunk(SCHEMA, '''      "required": [
        "kindId",
        "reason"
      ],
      "properties": {
        "kindId": {
          "type": "string",
          "minLength": 1,
          "maxLength": 256,
          "pattern": "^[A-Za-z0-9][A-Za-z0-9._:/-]*$"
        },
        "reason": {''', '''      "required": [
        "kindId",
        "standard",
        "reason"
      ],
      "properties": {
        "kindId": {
          "type": "string",
          "minLength": 1,
          "maxLength": 256,
          "pattern": "^[A-Za-z0-9][A-Za-z0-9._:/-]*$"
        },
        "standard": {
          "type": "string",
          "minLength": 1,
          "maxLength": 256,
          "pattern": "^[A-Za-z0-9][A-Za-z0-9._:/-]*$"
        },
        "reason": {''')
schema_text = text(SCHEMA)
request_def = schema_text[schema_text.index('    "SpaceArtifactCreationRequest": {'):]
request_def = request_def[:request_def.index('\n    },\n') + len('\n    },\n')]
assert '"kindId",\n        "name"' in request_def and '"kindId": {' in request_def
new_request = request_def.replace('"kindId",\n        "name"', '"kindId",\n        "dialect",\n        "name"')
kind_prop = new_request[new_request.index('        "kindId": {'):]
kind_prop = kind_prop[:kind_prop.index('\n        },\n') + len('\n        },\n')]
new_request = new_request.replace(kind_prop, kind_prop + '        "dialect": {\n          "description": "The chosen catalog entry\'s dialect: the entry of one standard of the kind.",\n          "$ref": "#/$defs/SpaceArtifactCreationCreationDialect"\n        },\n', 1)
hunk(SCHEMA, request_def, new_request)

# ── language-agnostic vectors (programmatic, minimal hunks) ───────────────────────────────────────────────────────────
vectors_text = text(VECTORS)
vectors = json.loads(vectors_text)
dialect_of = lambda kind: {'artifactKind': kind, 'standard': '1', 'subset': '*'}


def with_dialect(request):
    out = {}
    for key, value in request.items():
        out[key] = value
        if key == 'kindId':
            out['dialect'] = dialect_of(value if isinstance(value, str) and value else 's.fixture.document')
    return out


for row in vectors['requests']:
    if isinstance(row['value'], dict) and 'kindId' in row['value']:
        row['value'] = with_dialect(row['value'])
exact_request = next(row['value'] for row in vectors['requests'] if row['accepted'])
vectors['requests'] += [
    {'id': 'missing-dialect', 'value': {key: value for key, value in exact_request.items() if key != 'dialect'}, 'accepted': False},
    {'id': 'dialect-wildcard-standard', 'value': {**exact_request, 'dialect': {**exact_request['dialect'], 'standard': '*'}}, 'accepted': False},
]
for row in vectors['catalogs']:
    for withheld in row['value'].get('withheld', []):
        reordered = {'kindId': withheld['kindId'], 'standard': '1', 'reason': withheld['reason']}
        withheld.clear()
        withheld.update(reordered)
exact_catalog = next(row['value'] for row in vectors['catalogs'] if row['id'] == 'exact')
offered = exact_catalog['kinds'][0]
second = {**offered, 'dialect': {**offered['dialect'], 'standard': '2'}, 'label': {'en': offered['label']['en'] + ' 2', 'de': offered['label']['de'] + ' 2'}}
vectors['catalogs'] += [
    {'id': 'two-standards', 'value': {**exact_catalog, 'kinds': [offered, second]}, 'accepted': True},
    {'id': 'same-standard-twice', 'value': {**exact_catalog, 'kinds': [offered, {**offered, 'label': second['label']}]}, 'accepted': False, 'schemaValid': True},
    {'id': 'withheld-other-standard', 'value': {**exact_catalog, 'withheld': [{'kindId': offered['kindId'], 'standard': '2', 'reason': 'ambiguous-editors'}]}, 'accepted': True},
]
for row in vectors['rawJson']:
    if row['type'] == 'request' and '"kindId":' in row['source'] and '"dialect"' not in row['source']:
        row['source'] = re.sub(r'("kindId":"([^"]*)")', lambda match: f'{match.group(1)},"dialect":{{"artifactKind":"{match.group(2) or "s.fixture.document"}","standard":"1","subset":"*"}}', row['source'], count=1)
new_vectors = json.dumps(vectors, indent=2, ensure_ascii=False) + '\n'
H.append({'file': VECTORS, 'old': vectors_text, 'new': new_vectors})

# ── hub (anchored on A + 34) ──────────────────────────────────────────────────────────────────────────────────────────
hunk(CAT, r'''    /// is created by the whole-standard editor — the same app the guest answers `codec.genesis` with.
    pub(crate) fn artifact_creation_selection(&self, kind_id: &str) -> Option<&VerifiedDocumentOpenSelectionV1> {
        self.artifact_creation_choice(kind_id).one()
    }

    /// 🌱️ What the creation rule answers for `kind_id` ([`Self::artifact_creation_selection`]): its one creating editor,
    /// `Ambiguous` for several most general editors, `Empty` for a kind no editor opens.
    fn artifact_creation_choice(&self, kind_id: &str) -> MostGeneralDialect<&VerifiedDocumentOpenSelectionV1> {
        let (hosted, owned): (Vec<_>, Vec<_>) = self.open_targets.iter().filter(|selection| selection.artifact.kind == kind_id && selection.surface.role == DocumentOpenSurfaceRoleV1::Editor).partition(|selection| self.hosted_row(selection).is_some());
        owner_preferred_creation(owned, hosted, |selection| &selection.parent_dialect)
    }''', r'''    /// is created by the whole-standard editor — the same app the guest answers `codec.genesis` with. A kind whose editors span
    /// several standards is created once per standard ([`Self::artifact_creation_standards`]): the request names the entry by
    /// its dialect, and only that entry's own editor creates it.
    pub(crate) fn artifact_creation_selection(&self, kind_id: &str, dialect: &semio_framework::ArtifactDialect) -> Option<&VerifiedDocumentOpenSelectionV1> {
        self.artifact_creation_choice(kind_id, &dialect.standard).one().filter(|selection| selection.parent_dialect == *dialect)
    }

    /// 🏅️ The standards the CREATING editors of `kind_id` span ([`Self::creates_target_schema`]): creation offers one entry per
    /// standard.
    pub(crate) fn artifact_creation_standards(&self, kind_id: &str) -> BTreeSet<&str> {
        self.open_targets.iter().filter(|selection| selection.artifact.kind == kind_id && selection.surface.role == DocumentOpenSurfaceRoleV1::Editor && self.creates_target_schema(selection)).map(|selection| selection.parent_dialect.standard.as_str()).collect()
    }

    /// 🌱️ Whether `selection`'s editor CREATES documents of its target's schema: its app's own document schema
    /// (`AppIo::artifact_schema`, the schema the guest's `codec.genesis` answers with). An editor that also opens another schema
    /// of its kind (a hosted kind's second schema: gif `87a` opens `stdio.gif.89a` too) or a kind it declares beside its own
    /// (block's editors open `kit.catalog`) never creates it, so each creation entry has the one creator its genesis can answer.
    fn creates_target_schema(&self, selection: &VerifiedDocumentOpenSelectionV1) -> bool {
        self.selection_package(selection).and_then(|package| package.descriptor.manifest.apps.iter().find(|app| app.id == selection.surface.app_id)).is_some_and(|app| app.io.artifact_schema == selection.artifact.schema)
    }

    /// 🌱️ What the creation rule answers for `kind_id` within `standard` ([`Self::artifact_creation_selection`]): its one
    /// creating editor, `Ambiguous` for several most general editors of that standard, `Empty` for none.
    pub(crate) fn artifact_creation_choice(&self, kind_id: &str, standard: &str) -> MostGeneralDialect<&VerifiedDocumentOpenSelectionV1> {
        let (hosted, owned): (Vec<_>, Vec<_>) = self
            .open_targets
            .iter()
            .filter(|selection| selection.artifact.kind == kind_id && selection.parent_dialect.standard == standard && selection.surface.role == DocumentOpenSurfaceRoleV1::Editor && self.creates_target_schema(selection))
            .partition(|selection| self.hosted_row(selection).is_some());
        owner_preferred_creation(owned, hosted, |selection| &selection.parent_dialect)
    }''')

hunk(CAT, r'''    /// 🗣️ Projects every kind an editor of this generation opens from retained compiled descriptors: the offered choices, and
    /// each kind that is not offered NAMED with its typed reason — several most general editors (two standards), no
    /// declaration labelling it (a hosted kind is labelled by its owner's declaration; a host carries only `{ id, schema,
    /// owner }`), or a row that cannot be presented (a label over 128 characters). One such kind never withholds the others.''', r'''    /// 🗣️ Projects every (kind, standard) an editor of this generation opens from retained compiled descriptors: the offered
    /// entries — a kind whose editors span several standards is offered once per standard, labelled with it (en and de: "PDF
    /// 1.7") — and each entry that is not offered NAMED with its typed reason: several most general editors of one standard, no
    /// declaration labelling the kind (a hosted kind is labelled by its owner's declaration; a host carries only `{ id, schema,
    /// owner }`), or a row that cannot be presented (a label over 128 characters). One such entry never withholds the others.''')

hunk(CAT, r'''        let withhold = |kind_id: &str, reason| SpaceArtifactCreationWithheldKindV1 { kind_id: kind_id.into(), reason };
        for kind_id in kind_ids {
            let selection = match self.artifact_creation_choice(kind_id) {
                MostGeneralDialect::One(selection) => selection,
                MostGeneralDialect::Ambiguous => {
                    withheld.push(withhold(kind_id, SpaceArtifactCreationWithheldReasonV1::AmbiguousEditors));
                    continue;
                }
                MostGeneralDialect::Empty => continue,
            };''', r'''        let withhold = |kind_id: &str, standard: &str, reason| SpaceArtifactCreationWithheldKindV1 { kind_id: kind_id.into(), standard: standard.into(), reason };
        for (kind_id, standard, standards) in kind_ids.into_iter().flat_map(|kind_id| {
            let standards = self.artifact_creation_standards(kind_id);
            let count = standards.len();
            standards.into_iter().map(move |standard| (kind_id, standard, count))
        }) {
            let selection = match self.artifact_creation_choice(kind_id, standard) {
                MostGeneralDialect::One(selection) => selection,
                MostGeneralDialect::Ambiguous => {
                    withheld.push(withhold(kind_id, standard, SpaceArtifactCreationWithheldReasonV1::AmbiguousEditors));
                    continue;
                }
                MostGeneralDialect::Empty => continue,
            };''')

hunk(CAT, r'''                withheld.push(withhold(kind_id, SpaceArtifactCreationWithheldReasonV1::Unlabelled));
                continue;
            };''', r'''                withheld.push(withhold(kind_id, standard, SpaceArtifactCreationWithheldReasonV1::Unlabelled));
                continue;
            };
            let labelled = |label: &str| if standards > 1 { format!("{label} {standard}") } else { label.to_string() };''')

hunk(CAT, r'''                label: SpaceArtifactCreationLabelV1 {
                    en: kind.label.resolve(semio_framework::Terminology::Native, semio_framework::Locale::En).to_string(),
                    de: kind.label.resolve(semio_framework::Terminology::Native, semio_framework::Locale::De).to_string(),
                },
            };
            if choice.validate() {
                kinds.push(choice);
            } else {
                withheld.push(withhold(kind_id, SpaceArtifactCreationWithheldReasonV1::Unpresentable));
            }
        }
        kinds.sort_by(|left, right| left.kind_id.cmp(&right.kind_id));
        withheld.sort_by(|left, right| left.kind_id.cmp(&right.kind_id));''', r'''                label: SpaceArtifactCreationLabelV1 {
                    en: labelled(kind.label.resolve(semio_framework::Terminology::Native, semio_framework::Locale::En)),
                    de: labelled(kind.label.resolve(semio_framework::Terminology::Native, semio_framework::Locale::De)),
                },
            };
            if choice.validate() {
                kinds.push(choice);
            } else {
                withheld.push(withhold(kind_id, standard, SpaceArtifactCreationWithheldReasonV1::Unpresentable));
            }
        }
        kinds.sort_by(|left, right| (left.kind_id.as_str(), left.dialect.standard.as_str()).cmp(&(right.kind_id.as_str(), right.dialect.standard.as_str())));
        withheld.sort_by(|left, right| (left.kind_id.as_str(), left.standard.as_str()).cmp(&(right.kind_id.as_str(), right.standard.as_str())));''')

hunk(CAT, r'''    /// 🎯️ The one answer, when there is one.
    fn one(self) -> Option<T> {''', r'''    /// 🎯️ The one answer, when there is one.
    pub(crate) fn one(self) -> Option<T> {''')

hunk(CREATION, r'''/// 🪪️ Server-owned creation coordinates; no client supplies pair bytes or a descriptor.
pub struct ArtifactGenesisRequest {
    pub scope: DocumentScope,
    pub kind_id: String,
}''', r'''/// 🪪️ Server-owned creation coordinates; no client supplies pair bytes or a descriptor. `dialect` names the chosen catalog
/// entry (one per standard of the kind).
pub struct ArtifactGenesisRequest {
    pub scope: DocumentScope,
    pub kind_id: String,
    pub dialect: ArtifactDialect,
}

/// 🧭️ The dialect a create request names, as the catalog's own dialect type.
pub(crate) fn requested_creation_dialect(dialect: &directory::os_directory::schema::space_artifact_creation::SpaceArtifactCreationDialectV1) -> ArtifactDialect {
    ArtifactDialect { artifact_kind: dialect.artifact_kind.clone(), standard: dialect.standard.clone(), subset: dialect.subset.clone() }
}''')

hunk(CREATION, r'''        let selected = self.catalog.artifact_creation_selection(&request.kind_id).ok_or_else(|| AuthorityError::Catalog("creation kind has no unambiguous verified editor".into()))?;''', r'''        let selected = self.catalog.artifact_creation_selection(&request.kind_id, &request.dialect).ok_or_else(|| AuthorityError::Catalog("creation entry has no unambiguous verified editor".into()))?;''')

hunk(SERVICE, r'''            && self.catalog.artifact_creation_selection(&intent.request.kind_id).is_some_and(|selected| {''', r'''            && self.catalog.artifact_creation_selection(&intent.request.kind_id, &requested_creation_dialect(&intent.request.dialect)).is_some_and(|selected| {''')
hunk(SERVICE, r'''        let selected = self.catalog.artifact_creation_selection(&request.kind_id).ok_or_else(|| DirectoryError::Conflict("artifact creation kind has no verified native editor".into()))?;''', r'''        let selected = self.catalog.artifact_creation_selection(&request.kind_id, &requested_creation_dialect(&request.dialect)).ok_or_else(|| DirectoryError::Conflict("artifact creation entry has no verified native editor".into()))?;''')
hunk(SERVICE, r'''ArtifactGenesisRequest { scope: intent.scope.clone(), kind_id: intent.request.kind_id.clone() }''', r'''ArtifactGenesisRequest { scope: intent.scope.clone(), kind_id: intent.request.kind_id.clone(), dialect: requested_creation_dialect(&intent.request.dialect) }''')

hunk(INFERENCE, r'''/// `artifact_creation_selection` is the creation rule scoped to one kind: the owners' ONE most
/// general writable editor target whose codec identity this generation verified.
fn verified_gis_map_binding_with_service(catalog: Arc<VerifiedTrustedCatalog>, native: ArtifactInferenceService) -> Result<Option<Arc<VerifiedGisMapArtifactBindingV1>>, InferenceErrorV1> {
    let Some(selection) = catalog.artifact_creation_selection("s.gis.gismap") else { return Ok(None) };''', r'''/// `artifact_creation_choice` is the creation rule scoped to the kind's one standard: the owners'
/// ONE most general writable editor target whose codec identity this generation verified.
fn verified_gis_map_binding_with_service(catalog: Arc<VerifiedTrustedCatalog>, native: ArtifactInferenceService) -> Result<Option<Arc<VerifiedGisMapArtifactBindingV1>>, InferenceErrorV1> {
    let standards = catalog.artifact_creation_standards("s.gis.gismap");
    let Some(selection) = standards.iter().next().filter(|_| standards.len() == 1).and_then(|standard| catalog.artifact_creation_choice("s.gis.gismap", standard).one()) else { return Ok(None) };''')

hunk(CREATION_LAW, r'''        let mut request = ArtifactGenesisRequest { scope: expected.scope.clone(), kind_id: identity.artifact_kind.clone() };''', r'''        let mut request = ArtifactGenesisRequest { scope: expected.scope.clone(), kind_id: identity.artifact_kind.clone(), dialect: directory::os_pack::json::from_json_str(&fixture["dialect"].to_string()).unwrap() };''')
hunk(CREATION_LAW, r'''        let request = ArtifactGenesisRequest { scope: expected.scope.clone(), kind_id: identity.artifact_kind.clone() };''', r'''        let request = ArtifactGenesisRequest { scope: expected.scope.clone(), kind_id: identity.artifact_kind.clone(), dialect: directory::os_pack::json::from_json_str(&fixture["dialect"].to_string()).unwrap() };''')

operation_text = text(OPERATION)
operation = json.loads(operation_text)
request = operation['intent']['request']
operation['intent']['request'] = with_dialect(request)
operation['intent']['request']['dialect'] = operation['intent']['parentDialect']
canonical = json.dumps(operation['intent']['request'], separators=(',', ':'), ensure_ascii=False).encode()
space = operation['intent']['scope']['spaceId'].encode()
digest = hashlib.sha256(b'semio.hub.artifact-creation-intent.v1\0' + struct.pack('>Q', len(space)) + space + struct.pack('>Q', len(canonical)) + canonical).hexdigest()
old_digest = operation['intent']['commandSha256']
dialect_json = json.dumps(operation['intent']['parentDialect'], indent=2, ensure_ascii=False).replace('\n', '\n      ')
hunk(OPERATION, '      "kindId": "s.gis.map",\n      "name": "New Map"\n    },', '      "kindId": "s.gis.map",\n      "dialect": ' + dialect_json + ',\n      "name": "New Map"\n    },')
hunk(OPERATION, f'"commandSha256": "{old_digest}"', f'"commandSha256": "{digest}"')

bin_text = text(BIN_UNIT)
hunk(BIN_UNIT, r'''        kind_id: descriptor.artifact_kind.clone(),
        name: "Checkpoint publication fixture".into(),''', r'''        kind_id: descriptor.artifact_kind.clone(),
        dialect: directory::os_directory::schema::space_artifact_creation::SpaceArtifactCreationDialectV1 { artifact_kind: descriptor.artifact_kind.clone(), standard: "1".into(), subset: "*".into() },
        name: "Checkpoint publication fixture".into(),''')
hunk(BIN_UNIT, r'''            kind_id: "s.gis.gismap".into(),
            name: "Shared Map".into(),''', r'''            kind_id: "s.gis.gismap".into(),
            dialect: catalog.kinds[0].dialect.clone(),
            name: "Shared Map".into(),''')

hunk(CAT_LAW, r'''        let selection = catalog.artifact_creation_selection("s.fixture.document").expect("the hosted multi-subset kind is creatable");''', r'''        let whole = semio_framework::ArtifactDialect { artifact_kind: "s.fixture.document".into(), standard: "1".into(), subset: "*".into() };
        assert!(catalog.artifact_creation_selection("s.fixture.document", &semio_framework::ArtifactDialect { subset: "strict".into(), ..whole.clone() }).is_none(), "a subset editor never creates for its whole standard");
        let selection = catalog.artifact_creation_selection("s.fixture.document", &whole).expect("the hosted multi-subset kind is creatable");''')
hunk(CAT_LAW, r'''/// 🚫️ LAW: the Space creation catalog names every kind an editor of the generation opens — offered when one most general
/// editor creates it and its owner's declaration labels it presentably, otherwise WITHHELD with its typed reason: two most
/// general editors of two standards (`ambiguous-editors`), a label over 128 characters (`unpresentable`). Nothing is dropped
/// silently and one withheld kind never withholds the catalog (ticket 26/09/23 H14: the first hosted kind turned the whole
/// catalog into 409, and multi-standard kinds vanished without a word).''', r'''/// 🚫️ LAW: the Space creation catalog names every (kind, standard) an editor of the generation opens — offered when one most
/// general editor of that standard creates it and its owner's declaration labels it presentably (a kind of two standards is
/// offered twice, each labelled with its standard), otherwise WITHHELD with its typed reason (a label over 128 characters:
/// `unpresentable`). Nothing is dropped silently and one withheld entry never withholds the catalog (ticket 26/09/23 H14: the
/// first hosted kind turned the whole catalog into 409, and multi-standard kinds vanished without a word).''')
hunk(CAT_LAW, r'''    let named = |withheld: SpaceArtifactCreationWithheldReasonV1| vec![SpaceArtifactCreationWithheldKindV1 { kind_id: "s.fixture.document".into(), reason: withheld }];
    for (standard, subset, owner_label, offered, withheld) in [
        ("1", "strict", None, true, Vec::new()),
        ("2", "*", None, false, named(SpaceArtifactCreationWithheldReasonV1::AmbiguousEditors)),
        ("1", "strict", Some("x".repeat(129)), false, named(SpaceArtifactCreationWithheldReasonV1::Unpresentable)),
    ] {''', r'''    let named = |withheld: SpaceArtifactCreationWithheldReasonV1| vec![SpaceArtifactCreationWithheldKindV1 { kind_id: "s.fixture.document".into(), standard: "1".into(), reason: withheld }];
    for (standard, subset, owner_label, offered, withheld) in [
        ("1", "strict", None, vec![("1", "Fixture Document")], Vec::new()),
        ("2", "*", None, vec![("1", "Fixture Document 1"), ("2", "Fixture Document 2")], Vec::new()),
        ("1", "strict", Some("x".repeat(129)), Vec::new(), named(SpaceArtifactCreationWithheldReasonV1::Unpresentable)),
    ] {''')
hunk(CAT_LAW, r'''        assert_eq!(creation.kinds.iter().map(|kind| kind.kind_id.as_str()).collect::<Vec<_>>(), if offered { vec!["s.fixture.document"] } else { Vec::new() }, "{standard}/{subset}");''', r'''        assert!(creation.kinds.iter().all(|kind| kind.kind_id == "s.fixture.document" && kind.dialect.subset == "*"), "{standard}/{subset}: every entry is created by the most general editor of its standard");
        assert_eq!(creation.kinds.iter().map(|kind| (kind.dialect.standard.as_str(), kind.label.en.as_str())).collect::<Vec<_>>(), offered, "{standard}/{subset}");''')

# ── os TS worker wire, worker, shell ──────────────────────────────────────────────────────────────────────────────────
hunk(OS_TS, '''  if (kinds.some((entry, index) => index > 0 && kinds[index - 1]!.kindId >= entry.kindId)) throw new Error("space artifact creation catalog: invalid order");''', '''  const entryKey = (kindId: string, standard: string): string => `${kindId}\\u0000${standard}`;
  if (kinds.some((entry, index) => index > 0 && entryKey(kinds[index - 1]!.kindId, kinds[index - 1]!.dialect.standard) >= entryKey(entry.kindId, entry.dialect.standard))) throw new Error("space artifact creation catalog: invalid order");''')
hunk(OS_TS, '''    if (typeof value !== "object" || value === null || Array.isArray(value) || Object.keys(value).sort().join(",") !== "kindId,reason") throw new Error("space artifact creation catalog: invalid withheld kind");
    const entry = value as Readonly<Record<string, unknown>>;
    const kindId = workerWireCreationIdentityV1(entry.kindId),
      reason = SPACE_ARTIFACT_CREATION_WITHHELD_REASONS_V1.find((known) => known === entry.reason);
    if (kindId === null || reason === undefined) throw new Error("space artifact creation catalog: invalid withheld kind");
    return { kindId, reason };
  });
  if (withheld.some((entry, index) => (index > 0 && withheld[index - 1]!.kindId >= entry.kindId) || kinds.some((offered) => offered.kindId === entry.kindId)))''', '''    if (typeof value !== "object" || value === null || Array.isArray(value) || Object.keys(value).sort().join(",") !== "kindId,reason,standard") throw new Error("space artifact creation catalog: invalid withheld kind");
    const entry = value as Readonly<Record<string, unknown>>;
    const kindId = workerWireCreationIdentityV1(entry.kindId),
      standard = workerWireDialectComponentV1(entry.standard),
      reason = SPACE_ARTIFACT_CREATION_WITHHELD_REASONS_V1.find((known) => known === entry.reason);
    if (kindId === null || standard === null || reason === undefined) throw new Error("space artifact creation catalog: invalid withheld kind");
    return { kindId, standard, reason };
  });
  if (withheld.some((entry, index) => (index > 0 && entryKey(withheld[index - 1]!.kindId, withheld[index - 1]!.standard) >= entryKey(entry.kindId, entry.standard)) || kinds.some((offered) => offered.kindId === entry.kindId && offered.dialect.standard === entry.standard)))''')
hunk(OS_TS, '''  const expected = create ? "expectedCatalogGenerationId,kind,kindId,name,requestId,spaceId" : "kind,requestId,spaceId";''', '''  const expected = create ? "dialect,expectedCatalogGenerationId,kind,kindId,name,requestId,spaceId" : "kind,requestId,spaceId";''')
hunk(OS_TS, '''    kindId = workerWireCreationIdentityV1(parsed.kindId),
    name = workerWireTextV1(parsed.name);
  if (expectedCatalogGenerationId === null || kindId === null || name === null) throw new Error("backbone worker request: invalid space artifact creation intent");
  return { kind: "space-artifact-create", requestId, spaceId, expectedCatalogGenerationId, kindId, name };''', '''    kindId = workerWireCreationIdentityV1(parsed.kindId),
    dialect = typeof parsed.dialect === "object" && parsed.dialect !== null && !Array.isArray(parsed.dialect) && Object.keys(parsed.dialect).sort().join(",") === "artifactKind,standard,subset" ? (parsed.dialect as Readonly<Record<string, unknown>>) : null,
    artifactKind = dialect === null ? null : workerWireCreationIdentityV1(dialect.artifactKind),
    standard = dialect === null ? null : workerWireDialectComponentV1(dialect.standard),
    subset = dialect === null ? null : workerWireDialectComponentV1(dialect.subset),
    name = workerWireTextV1(parsed.name);
  if (expectedCatalogGenerationId === null || kindId === null || artifactKind === null || standard === null || subset === null || name === null) throw new Error("backbone worker request: invalid space artifact creation intent");
  return { kind: "space-artifact-create", requestId, spaceId, expectedCatalogGenerationId, kindId, dialect: { artifactKind, standard, subset }, name };''')
hunk(OS_TS, '''  | { readonly kind: "space-artifact-create"; readonly requestId: string; readonly spaceId: string; readonly expectedCatalogGenerationId: string; readonly kindId: string; readonly name: string }''', '''  | { readonly kind: "space-artifact-create"; readonly requestId: string; readonly spaceId: string; readonly expectedCatalogGenerationId: string; readonly kindId: string; readonly dialect: ArtifactDialect; readonly name: string }''')
hunk(WORKER, '''existing.request.kindId === request.kindId && existing.request.name === request.name''', '''existing.request.kindId === request.kindId && existing.request.dialect.artifactKind === request.dialect.artifactKind && existing.request.dialect.standard === request.dialect.standard && existing.request.dialect.subset === request.dialect.subset && existing.request.name === request.name''')
hunk(SHELL, '''    return { kind: "space-artifact-create", requestId, spaceId, expectedCatalogGenerationId: capturedCatalog.catalogGenerationId, kindId: choice.kindId, name: args.name };''', '''    return { kind: "space-artifact-create", requestId, spaceId, expectedCatalogGenerationId: capturedCatalog.catalogGenerationId, kindId: choice.kindId, dialect: choice.dialect, name: args.name };''')

hunk(OWNER_TEST, '''kindId: "s.gis.gismap", name: "''', f'''kindId: "s.gis.gismap", dialect: {GIS}, name: "''', all_occurrences=True)
hunk(ENVELOPE_TEST, '''kindId: "s.gis.gismap", name: "Shared Map" };''', f'''kindId: "s.gis.gismap", dialect: {GIS}, name: "Shared Map" }};''')
hunk(WORKER_SCHEMA, '''      "required": ["kind", "requestId", "spaceId", "expectedCatalogGenerationId", "kindId", "name"],
      "properties": {
        "kind": { "const": "space-artifact-create" },
        "requestId": { "$ref": "#/$defs/RequestId" },
        "spaceId": { "$ref": "#/$defs/Identity" },
        "expectedCatalogGenerationId": { "$ref": "#/$defs/Digest" },
        "kindId": { "$ref": "#/$defs/Identity" },''', '''      "required": ["kind", "requestId", "spaceId", "expectedCatalogGenerationId", "kindId", "dialect", "name"],
      "properties": {
        "kind": { "const": "space-artifact-create" },
        "requestId": { "$ref": "#/$defs/RequestId" },
        "spaceId": { "$ref": "#/$defs/Identity" },
        "expectedCatalogGenerationId": { "$ref": "#/$defs/Digest" },
        "kindId": { "$ref": "#/$defs/Identity" },
        "dialect": {
          "description": "The chosen catalog entry's dialect: one entry per standard of the kind.",
          "type": "object",
          "additionalProperties": false,
          "required": ["artifactKind", "standard", "subset"],
          "properties": {
            "artifactKind": { "$ref": "#/$defs/Identity" },
            "standard": { "$ref": "#/$defs/Identity" },
            "subset": { "anyOf": [{ "const": "*" }, { "$ref": "#/$defs/Identity" }] }
          }
        },''')
hunk(WORKER_FIXTURE, '''    "kindId": "s.gis.gismap",
    "name": "Shared Map"
  },''', '''    "kindId": "s.gis.gismap",
    "dialect": {
      "artifactKind": "s.gis.gismap",
      "standard": "1",
      "subset": "any"
    },
    "name": "Shared Map"
  },''')

hunk(WGPU, r'''        kind_id: kind.kind_id.clone(),
        name: creation.name_draft.trim_matches(' ').to_string(),''', r'''        kind_id: kind.kind_id.clone(),
        dialect: kind.dialect.clone(),
        name: creation.name_draft.trim_matches(' ').to_string(),''')
hunk(WGPU_UNIT, r'''kind_id: kind.kind_id.clone(), name: "Shared Map".into() };''', r'''kind_id: kind.kind_id.clone(), dialect: kind.dialect.clone(), name: "Shared Map".into() };''')

# ── TS clients and harnesses name the chosen entry's dialect ──────────────────────────────────────────────────────────
for file, old, new in [
    (f'{MCP}/💼️inference-quartet/🟦️.ts', 'kindId: kind.kindId, name })', 'kindId: kind.kindId, dialect: kind.dialect, name })'),
    (f'{MCP}/🧩️plugin-coverage/🟦️.ts', 'kindId: String(entry.kind.kindId), name', 'kindId: String(entry.kind.kindId), dialect: entry.kind.dialect, name'),
    (f'{MCP}/🤝️hub-edit-durability/🟦️.ts', 'kindId: String(kind?.kindId ?? ""), name', 'kindId: String(kind?.kindId ?? ""), dialect: kind?.dialect, name'),
    (f'{MCP}/🚶️user-path/🟦️.ts', 'kindId: String(kind.kindId), name', 'kindId: String(kind.kindId), dialect: kind.dialect, name'),
    (f'{MCP}/🛡️security/🟦️.ts', 'kindId: String(kind?.kindId ?? ""), name', 'kindId: String(kind?.kindId ?? ""), dialect: kind?.dialect, name'),
    (f'{MCP}/🤖️hub-agent-participant/🟦️.ts', 'kindId: String(kind?.kindId ?? ""), name', 'kindId: String(kind?.kindId ?? ""), dialect: kind?.dialect, name'),
    (f'{HUB_TESTS}/📈️document-growth/🟦️.ts', 'kindId: kind.kindId, name: `Growth', 'kindId: kind.kindId, dialect: kind.dialect, name: `Growth'),
    (f'{HUB_TESTS}/🤝️two-client-document/🟦️.ts', '          kindId: kind.kindId,\n          name: "Two Client Note",', '          kindId: kind.kindId,\n          dialect: kind.dialect,\n          name: "Two Client Note",'),
    (HUB_SCRIPT, '    kindId: fixture.artifact.kind,\n    name: "Verified GIS Map",', '    kindId: fixture.artifact.kind,\n    dialect: { artifactKind: fixture.artifact.kind, standard: "1", subset: "*" },\n    name: "Verified GIS Map",'),
    (HUB_SCRIPT, 'kindId: target.artifactKind, name: "Trusted GIS Bootstrap Probe Map"', 'kindId: target.artifactKind, dialect: target.parentDialect, name: "Trusted GIS Bootstrap Probe Map"'),
    (HARNESS, 'kinds: { kindId: string; schema: string }[] }> {', 'kinds: { kindId: string; schema: string; dialect: { artifactKind: string; standard: string; subset: string } }[] }> {'),
    (HARNESS, 'kinds: (answer.json?.kinds ?? []) as { kindId: string; schema: string }[] };', 'kinds: (answer.json?.kinds ?? []) as { kindId: string; schema: string; dialect: { artifactKind: string; standard: string; subset: string } }[] };'),
    (HARNESS, '/** 🌱️ Runs the server-owned creation of one `kindId` document to `ready` and answers its artifact id and duration. */\nexport async function hubProbeCreateArtifact(origin: string, token: string, spaceId: string, generationId: string, kindId: string, name: string,', '/** 🌱️ Runs the server-owned creation of one catalog entry (`kindId` in one standard, named by its `dialect`) to `ready` and answers\n * its artifact id and duration. */\nexport async function hubProbeCreateArtifact(origin: string, token: string, spaceId: string, generationId: string, entry: Readonly<{ kindId: string; dialect: { artifactKind: string; standard: string; subset: string } }>, name: string,'),
    (HARNESS, 'JSON.stringify(sealSpaceArtifactCreateV1({ requestId, expectedCatalogGenerationId: generationId, kindId, name })));', 'JSON.stringify(sealSpaceArtifactCreateV1({ requestId, expectedCatalogGenerationId: generationId, kindId: entry.kindId, dialect: entry.dialect, name })));'),
    (HARNESS, 'throw new Error(`creation of ${kindId} ended', 'throw new Error(`creation of ${entry.kindId}@${entry.dialect.standard} ended'),
    (f'{HUB_TESTS}/🤖️agent-ceiling/🟦️.ts', 'catalog.generationId, kind.kindId, "Agent ceiling")', 'catalog.generationId, kind, "Agent ceiling")'),
    (f'{HUB_TESTS}/🐳️docker-image/🟦️.ts', 'const kindId = catalog.kinds.find((row) => row.kindId === options.kind)?.kindId;\n  if (!kindId) throw', 'const kind = catalog.kinds.find((row) => row.kindId === options.kind);\n  if (!kind) throw'),
    (f'{HUB_TESTS}/🐳️docker-image/🟦️.ts', 'step(`client A creates a ${kindId} document`);\n  const created = await hubProbeCreateArtifact(options.origin, tokenA, spaceId, catalog.generationId, kindId, "Docker smoke");', 'step(`client A creates a ${kind.kindId} document`);\n  const created = await hubProbeCreateArtifact(options.origin, tokenA, spaceId, catalog.generationId, kind, "Docker smoke");'),
    (f'{HUB_TESTS}/🧠️residency/🟦️.ts', 'catalog.generationId, kind.kindId, `Residency', 'catalog.generationId, kind, `Residency'),
    (f'{HUB_TESTS}/💾️backup-restore/🟦️.ts', 'catalog.generationId, kind.kindId, `Backup drill', 'catalog.generationId, kind, `Backup drill'),
    (f'{HUB_TESTS}/💾️backup-restore/🟦️.ts', 'catalog.generationId, kind.kindId, `Shutdown drill', 'catalog.generationId, kind, `Shutdown drill'),
    (WGPU_COLLAB, 'catalog.generationId, kind.kindId, `wgpu', 'catalog.generationId, kind, `wgpu'),
]:
    hunk(file, old, new)

LAW_RS = '🌎️hub/🗿️artifact-authority/🌱️creation/🧪️tests/🔬️unit/🦀️.rs'
hunk(LAW_RS, 'SpaceArtifactCreationWithheldKindV1 { kind_id: "z".repeat(256), reason:', 'SpaceArtifactCreationWithheldKindV1 { kind_id: "z".repeat(256), standard: "1".into(), reason:')
hunk(CAT_LAW, r'''    manifest["hostedArtifactKinds"] = serde_json::json!([{ "id": "s.fixture.document", "schema": schema, "owner": "fixture.base" }]);
    let mut strict = manifest["apps"][0].clone();''', r'''    manifest["hostedArtifactKinds"] = serde_json::json!([{ "id": "s.fixture.document", "schema": schema, "owner": "fixture.base" }]);
    for app in manifest["apps"].as_array_mut().expect("fixture apps") {
        app["io"]["artifactSchema"] = schema.into();
    }
    let mut strict = manifest["apps"][0].clone();''')
CONTRACT = f'{OS}/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts'
hunk(CONTRACT, '''      kindId: "s.gis.gismap",
      name: "Shared Map",
    });''', '''      kindId: "s.gis.gismap",
      dialect: JSON.parse(choice).dialect,
      name: "Shared Map",
    });''')
hunk(CONTRACT, '''      kindId: "2d.drawing",
      name: "Plan",''', '''      kindId: "2d.drawing",
      dialect: { artifactKind: "s.draw.drawing", standard: "1", subset: "*" },
      name: "Plan",''')
hunk(f'{HUB_TESTS}/🐳️docker-image/🟦️.ts', 'return { spaceId, kindId, artifactId: created.artifactId,', 'return { spaceId, kindId: kind.kindId, artifactId: created.artifactId,')

hunk(CAT_LAW, r'''#[test]
fn every_committed_editor_that_edits_a_document_opens_a_kind_through_the_one_rule() {
    let mut pending = vec![PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../✏️s/🔌️plugins")];
    let mut descriptors = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(&directory).unwrap_or_else(|error| panic!("{}: {error}", directory.display())) {
            let path = entry.expect("plugin tree entry").path();
            if path.is_dir() && !matches!(path.file_name().and_then(|name| name.to_str()), Some("dist" | "target" | "node_modules")) {
                pending.push(path);
            } else if path.file_name().and_then(|name| name.to_str()) == Some("🛂️.descriptor.semio") {
                descriptors.push(path);
            }
        }
    }
    let linked = linked_codec_registries();''', r'''#[test]
fn every_committed_editor_that_edits_a_document_opens_a_kind_through_the_one_rule() {
    let descriptors = committed_descriptor_paths();
    let linked = linked_codec_registries();''')

hunk(CAT_LAW, r'''fn linked_codec_registries() -> std::collections::BTreeMap<&'static str, BTreeSet<(String, String)>> {''', r'''/// 🗂️ Every committed package descriptor under `✏️s/🔌️plugins` (build outputs excluded).
fn committed_descriptor_paths() -> Vec<PathBuf> {
    let mut pending = vec![PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../✏️s/🔌️plugins")];
    let mut descriptors = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(&directory).unwrap_or_else(|error| panic!("{}: {error}", directory.display())) {
            let path = entry.expect("plugin tree entry").path();
            if path.is_dir() && !matches!(path.file_name().and_then(|name| name.to_str()), Some("dist" | "target" | "node_modules")) {
                pending.push(path);
            } else if path.file_name().and_then(|name| name.to_str()) == Some("🛂️.descriptor.semio") {
                descriptors.push(path);
            }
        }
    }
    descriptors
}

/// 🏅️ LAW (census): every (kind, standard) an editor of a committed isolated package descriptor CREATES (its app's own document
/// schema is the target's) has ONE most general creating editor there ([`most_general_dialect`]), so creation offers every
/// standard of every creatable kind — dwg `ac1018` and `ac1024`, pdf `1.4` and `1.7`, gif `87a` and `89a` each get their own
/// entry instead of withholding the kind.
#[test]
fn every_committed_kind_and_standard_an_editor_opens_has_one_creating_editor() {
    let mut entries = 0usize;
    let mut ambiguous = Vec::new();
    for path in committed_descriptor_paths() {
        let descriptor = decode_package_descriptor(&std::fs::read(&path).expect("committed descriptor")).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        if descriptor.execution != semio_framework::ExecutionMode::Isolated {
            continue;
        }
        let mut editors = std::collections::BTreeMap::<(String, String), Vec<_>>::new();
        let creates = |target: &schema::TrustedDescriptorOpenTargetV1| descriptor.manifest.apps.iter().any(|app| app.id == target.app_id && app.io.artifact_schema == target.artifact_schema);
        for target in descriptor_open_targets(&descriptor).into_iter().filter(|target| matches!(target.role, TrustedBundleOpenRole::Editor) && creates(target)) {
            editors.entry((target.artifact_kind.clone(), target.parent_dialect.standard.clone())).or_default().push(target);
        }
        for ((kind, standard), candidates) in editors {
            entries += 1;
            if !matches!(most_general_dialect(candidates, |target| &target.parent_dialect), MostGeneralDialect::One(_)) {
                ambiguous.push(format!("{kind}@{standard} ({})", path.display()));
            }
        }
    }
    assert!(entries > 0, "no committed (kind, standard) with an editor found");
    assert!(ambiguous.is_empty(), "{} of {entries} (kind, standard) entries have no one creating editor: {ambiguous:#?}", ambiguous.len());
}

fn linked_codec_registries() -> std::collections::BTreeMap<&'static str, BTreeSet<(String, String)>> {''')

out = pathlib.Path(__file__).with_name('h14-creation-standard.json')
out.write_text(json.dumps(H, ensure_ascii=False, indent=1) + '\n')
print(len(H), 'hunks,', len({h['file'] for h in H}), 'files ->', out.name)
