#!/usr/bin/env python3
"""🧰️ One-off: writes h14-catalog-bound.json (t6-queue row 34) — the Space creation catalog complete by construction and
never silent: MAX_KINDS = the trusted catalog's open-target ceiling (1024) over offered + withheld kinds, MAX_BYTES derived
from the worst-case row, the hub asserts the relation and NAMES every kind an editor opens that it does not offer with a typed
reason (`ambiguous-editors`, `unlabelled`, `unpresentable`), the worker and the React shell carry and render them in en + de,
capacity / capacity + 1 and withheld laws on the Rust (serde_json oracle) and TS (Ajv, i18next) twins. Anchored on the
post-state of h14-creation-rule.py (hold A)."""
import difflib, json, pathlib

ROOT = pathlib.Path('/Users/ueli/Documents/semio')
OS = '🧰️framework/🛍️products/💻️os'
RS = f'{OS}/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🦀️.rs'
TS = f'{OS}/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts'
VECTORS = f'{OS}/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🔣️.json'
SCHEMA = f'{OS}/🔨️modules/📇️directory/🧬️schema/🔣️.json'
OS_TS = f'{OS}/🟦️.ts'
WORKER = f'{OS}/🔨️modules/🏪️store/👷️worker/🟦️.ts'
SHELL = f'{OS}/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx'
UI = f'{OS}/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🌱️artifact-creation/🟦️.tsx'
COPY = f'{OS}/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧫️fixtures/🌱️artifact-creation/🔣️.json'
CONTRACT = f'{OS}/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts'
OWNER_TEST = f'{OS}/🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts'
ENVELOPE_TEST = f'{OS}/🧪️tests/🧪️backbone-envelope-io/🟦️.ts'
WGPU_UNIT = f'{OS}/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️HubConnection/🧪️tests/🔬️wgpu-unit/🦀️.rs'
PROJECTION = f'{OS}/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔗️hub-projection-workspace/🦀️.rs'
CAT = '🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs'
CAT_LAW = '🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🔬️unit/🦀️.rs'
LAW_RS = '🌎️hub/🗿️artifact-authority/🌱️creation/🧪️tests/🔬️unit/🦀️.rs'
LAW_TS = '🌎️hub/🧪️tests/🌱️creation-progress/🟦️.ts'
H = []


def hunk(file, old, new):
    H.append({'file': file, 'old': old, 'new': new})


def diff_hunks(file, old_text, new_text, context=3):
    """🧩️ Minimal unique old → new hunks between two texts of one file."""
    a, b = old_text.splitlines(keepends=True), new_text.splitlines(keepends=True)
    while True:
        groups = [(''.join(a[g[0][1]:g[-1][2]]), ''.join(b[g[0][3]:g[-1][4]])) for g in difflib.SequenceMatcher(a=a, b=b, autojunk=False).get_grouped_opcodes(context)]
        if all(old_text.count(old) == 1 for old, _ in groups):
            break
        context += 2
    for old, new in groups:
        hunk(file, old, new)


# ── os-kernel Rust schema ─────────────────────────────────────────────────────────────────────────────────────────────
hunk(RS, r'''/// 🧮️ Maximum selected-current creation choices disclosed to one Space dialog.
pub const SPACE_ARTIFACT_CREATION_CATALOG_MAX_KINDS: usize = 64;

/// 🧯️ Maximum canonical selected-current creation catalog bytes.
pub const SPACE_ARTIFACT_CREATION_CATALOG_MAX_BYTES: usize = 65_536;
''', r'''/// 🧮️ Maximum kinds one Space's creation catalog names, offered and withheld together: the hub's trusted-catalog
/// open-target ceiling (`TRUSTED_CATALOG_MAX_OPEN_TARGETS`, which the hub asserts is not larger). Every kind the catalog
/// names is opened by at least one verified editor target, so one response always names every kind of its generation.
pub const SPACE_ARTIFACT_CREATION_CATALOG_MAX_KINDS: usize = 1024;

/// 🦴️ The canonical JSON of one offered creation choice with every value empty: the fixed bytes of the largest row.
const SPACE_ARTIFACT_CREATION_KIND_SKELETON: &str = r#"{"kindId":"","schema":"","dialect":{"artifactKind":"","standard":"","subset":""},"label":{"en":"","de":""}}"#;

/// 🦴️ The canonical JSON of a creation catalog with every value empty and no row: the fixed bytes of the envelope.
const SPACE_ARTIFACT_CREATION_CATALOG_SKELETON: &str = r#"{"schema":"semio.hub.space-artifact-creation-catalog/v1","spaceId":"","catalogGenerationId":"","kinds":[],"withheld":[]}"#;

/// 📐️ Maximum canonical bytes of one named kind and its separating comma — an offered choice, the larger row: five identities
/// of 256 bytes and two labels of 128 characters of at most four UTF-8 bytes each (a label refuses control characters, so
/// none escapes to `\u`).
pub const SPACE_ARTIFACT_CREATION_KIND_MAX_BYTES: usize = SPACE_ARTIFACT_CREATION_KIND_SKELETON.len() + 5 * 256 + 2 * 128 * 4 + 1;

/// 🧯️ Maximum canonical creation catalog bytes, derived: every named kind at its maximum and the envelope with a 256-byte
/// space id and the 64-hex generation.
pub const SPACE_ARTIFACT_CREATION_CATALOG_MAX_BYTES: usize = SPACE_ARTIFACT_CREATION_CATALOG_MAX_KINDS * SPACE_ARTIFACT_CREATION_KIND_MAX_BYTES + SPACE_ARTIFACT_CREATION_CATALOG_SKELETON.len() + 256 + 64;
''')

hunk(RS, r'''/// 🗂️ Bounded selected-current choices for one authenticated Space.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SpaceArtifactCreationCatalogV1 {
    pub schema: String,
    pub space_id: String,
    pub catalog_generation_id: String,
    pub kinds: Vec<SpaceArtifactCreationKindV1>,
}

impl SpaceArtifactCreationCatalogV1 {
    /// 🧬️ The list is nonempty, bounded, kind-sorted and duplicate-free.
    pub fn validate(&self) -> bool {
        self.schema == "semio.hub.space-artifact-creation-catalog/v1"
            && identity(&self.space_id)
            && digest(&self.catalog_generation_id)
            && !self.kinds.is_empty()
            && self.kinds.len() <= SPACE_ARTIFACT_CREATION_CATALOG_MAX_KINDS
            && self.kinds.iter().all(SpaceArtifactCreationKindV1::validate)
            && self.kinds.windows(2).all(|pair| pair[0].kind_id < pair[1].kind_id)
    }
''', r'''/// 🚫️ Why a kind an editor of the generation opens is not offered for creation — named, never dropped silently: several
/// equally general editors (two standards; none is chosen by order), no declaration labelling it, or a row that cannot be
/// presented (a label over 128 characters).
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
#[serde(rename_all = "kebab-case")]
#[value(rename_all = "kebab-case")]
pub enum SpaceArtifactCreationWithheldReasonV1 {
    AmbiguousEditors,
    Unlabelled,
    Unpresentable,
}

/// 🚫️ One kind of the generation the catalog names but does not offer, with its reason.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SpaceArtifactCreationWithheldKindV1 {
    pub kind_id: String,
    pub reason: SpaceArtifactCreationWithheldReasonV1,
}

/// 🗂️ Every kind one authenticated Space's current generation names for creation: the offered choices and the withheld kinds.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SpaceArtifactCreationCatalogV1 {
    pub schema: String,
    pub space_id: String,
    pub catalog_generation_id: String,
    pub kinds: Vec<SpaceArtifactCreationKindV1>,
    pub withheld: Vec<SpaceArtifactCreationWithheldKindV1>,
}

impl SpaceArtifactCreationCatalogV1 {
    /// 🧬️ Offered and withheld kinds are each kind-sorted and duplicate-free, disjoint, together nonempty and bounded.
    pub fn validate(&self) -> bool {
        self.schema == "semio.hub.space-artifact-creation-catalog/v1"
            && identity(&self.space_id)
            && digest(&self.catalog_generation_id)
            && !(self.kinds.is_empty() && self.withheld.is_empty())
            && self.kinds.len() + self.withheld.len() <= SPACE_ARTIFACT_CREATION_CATALOG_MAX_KINDS
            && self.kinds.iter().all(SpaceArtifactCreationKindV1::validate)
            && self.kinds.windows(2).all(|pair| pair[0].kind_id < pair[1].kind_id)
            && self.withheld.iter().all(|row| identity(&row.kind_id))
            && self.withheld.windows(2).all(|pair| pair[0].kind_id < pair[1].kind_id)
            && self.withheld.iter().all(|row| self.kinds.binary_search_by(|kind| kind.kind_id.as_str().cmp(row.kind_id.as_str())).is_err())
    }
''')

# ── TS twin ───────────────────────────────────────────────────────────────────────────────────────────────────────────
hunk(TS, r'''export const SPACE_ARTIFACT_CREATION_MAX_BYTES = 4096;
export const SPACE_ARTIFACT_CREATION_CATALOG_MAX_BYTES = 65_536;
export const SPACE_ARTIFACT_CREATION_CATALOG_MAX_KINDS = 64;
''', r'''export const SPACE_ARTIFACT_CREATION_MAX_BYTES = 4096;
/** 🧮️ Every kind one generation names, offered and withheld: the hub's trusted-catalog open-target ceiling (Rust twin). */
export const SPACE_ARTIFACT_CREATION_CATALOG_MAX_KINDS = 1024;
/** 🦴️ The canonical JSON of one offered creation choice with every value empty: the fixed bytes of the largest row. */
const SPACE_ARTIFACT_CREATION_KIND_SKELETON = '{"kindId":"","schema":"","dialect":{"artifactKind":"","standard":"","subset":""},"label":{"en":"","de":""}}';
/** 🦴️ The canonical JSON of a creation catalog with every value empty and no row: the fixed bytes of the envelope. */
const SPACE_ARTIFACT_CREATION_CATALOG_SKELETON = '{"schema":"semio.hub.space-artifact-creation-catalog/v1","spaceId":"","catalogGenerationId":"","kinds":[],"withheld":[]}';
/** 📐️ One named kind and its comma at most: five 256-byte identities, two 128-character labels of at most four UTF-8 bytes each. */
export const SPACE_ARTIFACT_CREATION_KIND_MAX_BYTES = SPACE_ARTIFACT_CREATION_KIND_SKELETON.length + 5 * 256 + 2 * 128 * 4 + 1;
/** 🧯️ The derived catalog ceiling: every named kind at its maximum and the envelope with a 256-byte space id and the generation. */
export const SPACE_ARTIFACT_CREATION_CATALOG_MAX_BYTES = SPACE_ARTIFACT_CREATION_CATALOG_MAX_KINDS * SPACE_ARTIFACT_CREATION_KIND_MAX_BYTES + SPACE_ARTIFACT_CREATION_CATALOG_SKELETON.length + 256 + 64;
/** 🚫️ Why a kind an editor opens is not offered for creation (Rust twin `SpaceArtifactCreationWithheldReasonV1`). */
export const SPACE_ARTIFACT_CREATION_WITHHELD_REASONS_V1 = ["ambiguous-editors", "unlabelled", "unpresentable"] as const;
''')

hunk(TS, r'''export type SpaceArtifactCreationCatalogV1 = Readonly<{
  schema: typeof SPACE_ARTIFACT_CREATION_CATALOG_SCHEMA_V1;
  spaceId: string;
  catalogGenerationId: string;
  kinds: readonly SpaceArtifactCreationKindV1[];
}>;
''', r'''export type SpaceArtifactCreationWithheldReasonV1 = (typeof SPACE_ARTIFACT_CREATION_WITHHELD_REASONS_V1)[number];

export type SpaceArtifactCreationWithheldKindV1 = Readonly<{
  kindId: string;
  reason: SpaceArtifactCreationWithheldReasonV1;
}>;

export type SpaceArtifactCreationCatalogV1 = Readonly<{
  schema: typeof SPACE_ARTIFACT_CREATION_CATALOG_SCHEMA_V1;
  spaceId: string;
  catalogGenerationId: string;
  kinds: readonly SpaceArtifactCreationKindV1[];
  withheld: readonly SpaceArtifactCreationWithheldKindV1[];
}>;
''')

hunk(TS, r'''/** 📥️ Seals the only client-supplied creation fields. */''', r'''function withheldKind(value: unknown): SpaceArtifactCreationWithheldKindV1 | null {
  const row = record(value);
  if (row === null || !exactFields(row, ["kindId", "reason"])) return null;
  const kindId = identity(row.kindId),
    reason = SPACE_ARTIFACT_CREATION_WITHHELD_REASONS_V1.find((known) => known === row.reason);
  return kindId !== null && reason !== undefined ? { kindId, reason } : null;
}

/** 📥️ Seals the only client-supplied creation fields. */''')

hunk(TS, r'''  if (row === null || !exactFields(row, ["schema", "spaceId", "catalogGenerationId", "kinds"])) throw new Error("space artifact creation catalog: invalid fields");
  const spaceId = identity(row.spaceId),
    catalogGenerationId = digest(row.catalogGenerationId);
  if (row.schema !== SPACE_ARTIFACT_CREATION_CATALOG_SCHEMA_V1 || spaceId === null || catalogGenerationId === null || !Array.isArray(row.kinds) || row.kinds.length === 0 || row.kinds.length > SPACE_ARTIFACT_CREATION_CATALOG_MAX_KINDS)
    throw new Error("space artifact creation catalog: invalid owner");
  const kinds = row.kinds.map(creationKind);
  if (kinds.some((kind) => kind === null)) throw new Error("space artifact creation catalog: invalid kind");
  const sealed = kinds as SpaceArtifactCreationKindV1[];
  if (sealed.some((kind, index) => index > 0 && sealed[index - 1]!.kindId >= kind.kindId)) throw new Error("space artifact creation catalog: invalid order");
  const result: SpaceArtifactCreationCatalogV1 = { schema: SPACE_ARTIFACT_CREATION_CATALOG_SCHEMA_V1, spaceId, catalogGenerationId, kinds: sealed };''', r'''  if (row === null || !exactFields(row, ["schema", "spaceId", "catalogGenerationId", "kinds", "withheld"])) throw new Error("space artifact creation catalog: invalid fields");
  const spaceId = identity(row.spaceId),
    catalogGenerationId = digest(row.catalogGenerationId);
  if (
    row.schema !== SPACE_ARTIFACT_CREATION_CATALOG_SCHEMA_V1 || spaceId === null || catalogGenerationId === null || !Array.isArray(row.kinds) || !Array.isArray(row.withheld)
    || row.kinds.length + row.withheld.length === 0 || row.kinds.length + row.withheld.length > SPACE_ARTIFACT_CREATION_CATALOG_MAX_KINDS
  )
    throw new Error("space artifact creation catalog: invalid owner");
  const kinds = row.kinds.map(creationKind);
  if (kinds.some((kind) => kind === null)) throw new Error("space artifact creation catalog: invalid kind");
  const sealed = kinds as SpaceArtifactCreationKindV1[];
  if (sealed.some((kind, index) => index > 0 && sealed[index - 1]!.kindId >= kind.kindId)) throw new Error("space artifact creation catalog: invalid order");
  const withheld = row.withheld.map(withheldKind);
  if (withheld.some((kind) => kind === null)) throw new Error("space artifact creation catalog: invalid withheld kind");
  const named = withheld as SpaceArtifactCreationWithheldKindV1[];
  if (named.some((kind, index) => (index > 0 && named[index - 1]!.kindId >= kind.kindId) || sealed.some((offered) => offered.kindId === kind.kindId)))
    throw new Error("space artifact creation catalog: invalid withheld order");
  const result: SpaceArtifactCreationCatalogV1 = { schema: SPACE_ARTIFACT_CREATION_CATALOG_SCHEMA_V1, spaceId, catalogGenerationId, kinds: sealed, withheld: named };''')

# ── JSON Schema ───────────────────────────────────────────────────────────────────────────────────────────────────────
hunk(SCHEMA, '''        "catalogGenerationId",
        "kinds"
      ],
      "properties": {
        "schema": {
          "const": "semio.hub.space-artifact-creation-catalog/v1"''', '''        "catalogGenerationId",
        "kinds",
        "withheld"
      ],
      "anyOf": [
        {
          "properties": {
            "kinds": {
              "minItems": 1
            }
          }
        },
        {
          "properties": {
            "withheld": {
              "minItems": 1
            }
          }
        }
      ],
      "properties": {
        "schema": {
          "const": "semio.hub.space-artifact-creation-catalog/v1"''')

hunk(SCHEMA, '''        "kinds": {
          "type": "array",
          "minItems": 1,
          "maxItems": 64,
          "items": {
            "$ref": "#/$defs/SpaceArtifactCreationCreationKind"
          }
        }
      }
    },''', '''        "kinds": {
          "description": "The offered kinds, sorted by kind id. Offered and withheld kinds together name every kind an editor of the generation opens: at most the hub's trusted-catalog open-target ceiling, so a catalog is never cut off.",
          "type": "array",
          "minItems": 0,
          "maxItems": 1024,
          "items": {
            "$ref": "#/$defs/SpaceArtifactCreationCreationKind"
          }
        },
        "withheld": {
          "description": "The kinds an editor of the generation opens that are not offered, each with its reason, sorted by kind id and disjoint from `kinds`.",
          "type": "array",
          "minItems": 0,
          "maxItems": 1024,
          "items": {
            "$ref": "#/$defs/SpaceArtifactCreationWithheldKind"
          }
        }
      }
    },''')

hunk(SCHEMA, '''    "SpaceArtifactCreationCreationDialect": {''', '''    "SpaceArtifactCreationWithheldKind": {
      "type": "object",
      "additionalProperties": false,
      "required": [
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
        "reason": {
          "$ref": "#/$defs/SpaceArtifactCreationWithheldReason"
        }
      }
    },
    "SpaceArtifactCreationWithheldReason": {
      "description": "Why a kind an editor opens is not offered for creation: `ambiguous-editors` — several equally general editors (two standards; none is chosen by order); `unlabelled` — no declaration labels the kind; `unpresentable` — its row cannot be presented (a label over 128 characters).",
      "enum": [
        "ambiguous-editors",
        "unlabelled",
        "unpresentable"
      ]
    },
    "SpaceArtifactCreationCreationDialect": {''')

hunk(SCHEMA, '''        "opening",
        "phases",
        "catalog"
      ],''', '''        "opening",
        "phases",
        "catalog",
        "withheld"
      ],''')

hunk(SCHEMA, '''            "unavailable": {
              "type": "string",
              "minLength": 1,
              "maxLength": 512
            }
          }
        },
        "phases": {''', '''            "unavailable": {
              "type": "string",
              "minLength": 1,
              "maxLength": 512
            }
          }
        },
        "withheld": {
          "$ref": "#/$defs/ArtifactCreationWithheldCopyV1"
        },
        "phases": {''')

hunk(SCHEMA, '''    "ArtifactCreationProgressCopyV1": {''', '''    "ArtifactCreationWithheldCopyV1": {
      "description": "🚫️ The creation catalog's withheld kinds for one locale: a list heading and one line per reason, each naming the kind as `{kind}`.",
      "type": "object",
      "additionalProperties": false,
      "required": [
        "heading",
        "ambiguous-editors",
        "unlabelled",
        "unpresentable"
      ],
      "properties": {
        "heading": {
          "type": "string",
          "minLength": 1,
          "maxLength": 256
        },
        "ambiguous-editors": {
          "type": "string",
          "minLength": 1,
          "maxLength": 512,
          "pattern": "\\\\{kind\\\\}"
        },
        "unlabelled": {
          "type": "string",
          "minLength": 1,
          "maxLength": 512,
          "pattern": "\\\\{kind\\\\}"
        },
        "unpresentable": {
          "type": "string",
          "minLength": 1,
          "maxLength": 512,
          "pattern": "\\\\{kind\\\\}"
        }
      }
    },
    "ArtifactCreationProgressCopyV1": {''')

# ── fixture vectors (programmatic, minimal hunks) ─────────────────────────────────────────────────────────────────────
vectors_text = (ROOT / VECTORS).read_text()
vectors = json.loads(vectors_text)
for row in vectors['catalogs']:
    value = row['value']
    if isinstance(value, dict) and 'kinds' in value:
        row['value'] = {**value, 'withheld': value.get('withheld', [])}
    if row['id'] in ('unsorted', 'duplicate'):
        row['schemaValid'] = True
exact = next(row['value'] for row in vectors['catalogs'] if row['id'] == 'exact')
offered = exact['kinds'][0]['kindId']
withheld_rows = lambda *rows: [{'kindId': kind, 'reason': reason} for kind, reason in rows]
vectors['catalogs'] += [
    {'id': 'withheld-named', 'value': {**exact, 'withheld': withheld_rows(('zz.gif', 'ambiguous-editors'))}, 'accepted': True},
    {'id': 'withheld-only', 'value': {**exact, 'kinds': [], 'withheld': withheld_rows(('zz.dwg', 'ambiguous-editors'), ('zz.gif', 'unlabelled'), ('zz.pdf', 'unpresentable'))}, 'accepted': True},
    {'id': 'withheld-offered-too', 'value': {**exact, 'withheld': withheld_rows((offered, 'unlabelled'))}, 'accepted': False, 'schemaValid': True},
    {'id': 'withheld-unsorted', 'value': {**exact, 'kinds': [], 'withheld': withheld_rows(('zz.gif', 'unlabelled'), ('zz.dwg', 'ambiguous-editors'))}, 'accepted': False, 'schemaValid': True},
    {'id': 'withheld-unknown-reason', 'value': {**exact, 'withheld': withheld_rows(('zz.gif', 'hidden'))}, 'accepted': False},
]
for row in vectors['rawJson']:
    if row['type'] == 'catalog' and '"withheld"' not in row['source']:
        assert row['source'].endswith('}')
        row['source'] = row['source'][:-1] + ',"withheld":[]}'
diff_hunks(VECTORS, vectors_text, json.dumps(vectors, indent=2, ensure_ascii=False) + '\n')

# ── os TS worker wire ─────────────────────────────────────────────────────────────────────────────────────────────────
hunk(OS_TS, '''import { closeHubSocketV1 } from "./🔨️modules/📇️directory/🔌️client/🚪️socket-close/🟦️.ts";''', '''import { closeHubSocketV1 } from "./🔨️modules/📇️directory/🔌️client/🚪️socket-close/🟦️.ts";
import { SPACE_ARTIFACT_CREATION_CATALOG_MAX_KINDS, SPACE_ARTIFACT_CREATION_WITHHELD_REASONS_V1, type SpaceArtifactCreationWithheldKindV1 } from "./🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts";''')

hunk(OS_TS, '''  if (Object.keys(row).sort().join(",") !== "catalogGenerationId,clientInstanceId,kind,kinds,spaceId") throw new Error("space artifact creation catalog: invalid fields");
  const spaceId = workerWireCreationIdentityV1(row.spaceId),
    clientInstanceId = workerWireClientInstanceIdV1(row.clientInstanceId);
  const catalogGenerationId = workerWireCatalogGenerationIdV1(row.catalogGenerationId);
  if (row.kind !== "space-artifact-creation-catalog" || spaceId === null || clientInstanceId === null || catalogGenerationId === null || !Array.isArray(row.kinds) || row.kinds.length === 0 || row.kinds.length > 64)
    throw new Error("space artifact creation catalog: invalid owner");''', '''  if (Object.keys(row).sort().join(",") !== "catalogGenerationId,clientInstanceId,kind,kinds,spaceId,withheld") throw new Error("space artifact creation catalog: invalid fields");
  const spaceId = workerWireCreationIdentityV1(row.spaceId),
    clientInstanceId = workerWireClientInstanceIdV1(row.clientInstanceId);
  const catalogGenerationId = workerWireCatalogGenerationIdV1(row.catalogGenerationId);
  if (
    row.kind !== "space-artifact-creation-catalog" || spaceId === null || clientInstanceId === null || catalogGenerationId === null || !Array.isArray(row.kinds) || !Array.isArray(row.withheld)
    || row.kinds.length + row.withheld.length === 0 || row.kinds.length + row.withheld.length > SPACE_ARTIFACT_CREATION_CATALOG_MAX_KINDS
  )
    throw new Error("space artifact creation catalog: invalid owner");''')

hunk(OS_TS, '''  if (kinds.some((entry, index) => index > 0 && kinds[index - 1]!.kindId >= entry.kindId)) throw new Error("space artifact creation catalog: invalid order");
  return { kind: "space-artifact-creation-catalog", clientInstanceId, spaceId, catalogGenerationId, kinds };
}''', '''  if (kinds.some((entry, index) => index > 0 && kinds[index - 1]!.kindId >= entry.kindId)) throw new Error("space artifact creation catalog: invalid order");
  const withheld = row.withheld.map((value): SpaceArtifactCreationWithheldKindV1 => {
    if (typeof value !== "object" || value === null || Array.isArray(value) || Object.keys(value).sort().join(",") !== "kindId,reason") throw new Error("space artifact creation catalog: invalid withheld kind");
    const entry = value as Readonly<Record<string, unknown>>;
    const kindId = workerWireCreationIdentityV1(entry.kindId),
      reason = SPACE_ARTIFACT_CREATION_WITHHELD_REASONS_V1.find((known) => known === entry.reason);
    if (kindId === null || reason === undefined) throw new Error("space artifact creation catalog: invalid withheld kind");
    return { kindId, reason };
  });
  if (withheld.some((entry, index) => (index > 0 && withheld[index - 1]!.kindId >= entry.kindId) || kinds.some((offered) => offered.kindId === entry.kindId)))
    throw new Error("space artifact creation catalog: invalid withheld order");
  return { kind: "space-artifact-creation-catalog", clientInstanceId, spaceId, catalogGenerationId, kinds, withheld };
}''')

hunk(OS_TS, '''/** 🗂️ Current trusted creation choices for one authenticated Space. */
export type SpaceArtifactCreationCatalogV1 = Readonly<{
  kind: "space-artifact-creation-catalog";
  clientInstanceId: string;
  spaceId: string;
  catalogGenerationId: string;
  kinds: readonly SpaceArtifactCreationKindV1[];
}>;''', '''/** 🗂️ Current trusted creation choices for one authenticated Space, and the kinds its generation names but withholds. */
export type SpaceArtifactCreationCatalogV1 = Readonly<{
  kind: "space-artifact-creation-catalog";
  clientInstanceId: string;
  spaceId: string;
  catalogGenerationId: string;
  kinds: readonly SpaceArtifactCreationKindV1[];
  withheld: readonly SpaceArtifactCreationWithheldKindV1[];
}>;''')

hunk(WORKER, '''  return { kind: "space-artifact-creation-catalog", clientInstanceId, spaceId, catalogGenerationId: catalog.catalogGenerationId, kinds: catalog.kinds };''', '''  return { kind: "space-artifact-creation-catalog", clientInstanceId, spaceId, catalogGenerationId: catalog.catalogGenerationId, kinds: catalog.kinds, withheld: catalog.withheld };''')

# ── React shell ───────────────────────────────────────────────────────────────────────────────────────────────────────
hunk(SHELL, '''export type SpaceArtifactCreationCatalogProjectionV1 = Readonly<{
  phase: "loading" | "ready" | "unavailable";
  kinds: readonly ArtifactKindChoice[];''', '''export type SpaceArtifactCreationCatalogProjectionV1 = Readonly<{
  phase: "loading" | "ready" | "unavailable";
  kinds: readonly ArtifactKindChoice[];
  withheld: Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-catalog" }>["withheld"];''')

hunk(SHELL, '''    phase: effectivePhase,
    kinds: authority === null ? [] : catalog!.kinds,''', '''    phase: effectivePhase,
    kinds: authority === null ? [] : catalog!.kinds,
    withheld: authority === null ? [] : catalog!.withheld,''')

hunk(SHELL, '''<ArtifactCreationCatalogNotice status={creationCatalog} locale={uiLocale} hasChoices={creationCatalog.kinds.length > 0} />''', '''<ArtifactCreationCatalogNotice status={creationCatalog} locale={uiLocale} hasChoices={creationCatalog.kinds.length > 0} withheld={creationCatalog.withheld} />''')

hunk(UI, '''    catalog: {
      loading: "Loading the available artifact kinds…",
      ready: "The available artifact kinds are current.",
      unavailable: "Artifact kinds are unavailable. Reopen the space before creating an artifact.",
    },
  },''', '''    catalog: {
      loading: "Loading the available artifact kinds…",
      ready: "The available artifact kinds are current.",
      unavailable: "Artifact kinds are unavailable. Reopen the space before creating an artifact.",
    },
    withheld: {
      heading: "Artifact kinds that cannot be created here",
      "ambiguous-editors": "{kind}: several editors are equally general (for example two standards), so none is chosen.",
      unlabelled: "{kind}: no package names this artifact kind.",
      unpresentable: "{kind}: its name cannot be shown.",
    },
  },''')

hunk(UI, '''    catalog: {
      loading: "Verfügbare Artefaktarten werden geladen…",
      ready: "Die verfügbaren Artefaktarten sind aktuell.",
      unavailable: "Artefaktarten sind nicht verfügbar. Öffne den Space erneut, bevor du ein Artefakt erstellst.",
    },
  },''', '''    catalog: {
      loading: "Verfügbare Artefaktarten werden geladen…",
      ready: "Die verfügbaren Artefaktarten sind aktuell.",
      unavailable: "Artefaktarten sind nicht verfügbar. Öffne den Space erneut, bevor du ein Artefakt erstellst.",
    },
    withheld: {
      heading: "Artefaktarten, die hier nicht erstellt werden können",
      "ambiguous-editors": "{kind}: Mehrere Editoren sind gleich allgemein (zum Beispiel zwei Standards), daher wird keiner gewählt.",
      unlabelled: "{kind}: Kein Paket benennt diese Artefaktart.",
      unpresentable: "{kind}: Ihr Name kann nicht angezeigt werden.",
    },
  },''')

hunk(UI, '''/** ♿ Treats an impossible Ready-without-members presentation as unavailable authority. */
export function ArtifactCreationCatalogNotice({ status, locale, hasChoices }: Readonly<{ status: Pick<SpaceArtifactCreationCatalogStatusV1, "phase">; locale: string; hasChoices?: boolean }>): React.ReactElement | null {
  const language = artifactCreationProgressLocaleV1(locale);
  if (language === null) return null;
  const phase = status.phase === "ready" && hasChoices === false ? "unavailable" : status.phase;
  const text = ARTIFACT_CREATION_PROGRESS_TEXT_V1[language].catalog[phase];''', '''/** ♿ Treats an impossible Ready-without-members presentation as unavailable authority, and lists every kind the catalog names
 * but withholds with its reason, so no kind silently disappears from creation. */
export function ArtifactCreationCatalogNotice({
  status,
  locale,
  hasChoices,
  withheld = [],
}: Readonly<{ status: Pick<SpaceArtifactCreationCatalogStatusV1, "phase">; locale: string; hasChoices?: boolean; withheld?: Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-catalog" }>["withheld"] }>): React.ReactElement | null {
  const language = artifactCreationProgressLocaleV1(locale);
  if (language === null) return null;
  const phase = status.phase === "ready" && hasChoices === false ? "unavailable" : status.phase;
  const text = ARTIFACT_CREATION_PROGRESS_TEXT_V1[language].catalog[phase];
  const copy = ARTIFACT_CREATION_PROGRESS_TEXT_V1[language].withheld;''')

hunk(UI, '''      role={unavailable ? "alert" : "status"}
    >
      {text}
    </section>
  );
}''', '''      role={unavailable ? "alert" : "status"}
    >
      {text}
      {withheld.length > 0 ? (
        <ul aria-label={copy.heading} data-semio-artifact-creation-withheld={withheld.length}>
          {withheld.map((row) => (
            <li data-semio-artifact-creation-withheld-reason={row.reason} key={row.kindId}>
              {copy[row.reason].replace("{kind}", row.kindId)}
            </li>
          ))}
        </ul>
      ) : null}
    </section>
  );
}''')

hunk(COPY, '''        "unavailable": "Artifact kinds are unavailable. Reopen the space before creating an artifact."
      },''', '''        "unavailable": "Artifact kinds are unavailable. Reopen the space before creating an artifact."
      },
      "withheld": {
        "heading": "Artifact kinds that cannot be created here",
        "ambiguous-editors": "{kind}: several editors are equally general (for example two standards), so none is chosen.",
        "unlabelled": "{kind}: no package names this artifact kind.",
        "unpresentable": "{kind}: its name cannot be shown."
      },''')

hunk(COPY, '''        "unavailable": "Artefaktarten sind nicht verfügbar. Öffne den Space erneut, bevor du ein Artefakt erstellst."
      },''', '''        "unavailable": "Artefaktarten sind nicht verfügbar. Öffne den Space erneut, bevor du ein Artefakt erstellst."
      },
      "withheld": {
        "heading": "Artefaktarten, die hier nicht erstellt werden können",
        "ambiguous-editors": "{kind}: Mehrere Editoren sind gleich allgemein (zum Beispiel zwei Standards), daher wird keiner gewählt.",
        "unlabelled": "{kind}: Kein Paket benennt diese Artefaktart.",
        "unpresentable": "{kind}: Ihr Name kann nicht angezeigt werden."
      },''')

hunk(CONTRACT, '''      expect(region.getAttribute("data-semio-artifact-creation-catalog"), row.id).toBe(textKey);
      view.unmount();
    }''', '''      expect(region.getAttribute("data-semio-artifact-creation-catalog"), row.id).toBe(textKey);
      view.unmount();
    }
    for (const locale of ["en", "de"] as const) {
      const withheld = [
        { kindId: "s.stdio.dwg", reason: "ambiguous-editors" as const },
        { kindId: "s.stdio.gif", reason: "unlabelled" as const },
        { kindId: "s.stdio.pdf", reason: "unpresentable" as const },
      ];
      const view = render(createElement(ArtifactCreationCatalogNotice, { status: { phase: "ready" }, hasChoices: true, locale, withheld }));
      const list = view.getByRole("list", { name: artifactCreationProgressFixture.locales[locale].withheld.heading });
      expect(list.getAttribute("data-semio-artifact-creation-withheld"), locale).toBe(String(withheld.length));
      const items = [...list.querySelectorAll("li")];
      expect(items.map((item) => item.getAttribute("data-semio-artifact-creation-withheld-reason")), locale).toEqual(withheld.map((row) => row.reason));
      expect(items.map((item) => item.textContent), locale).toEqual(withheld.map((row) => (oracle.t(`withheld.${row.reason}`, { lng: locale }) as string).replace("{kind}", row.kindId)));
      view.unmount();
    }''')

# ── existing literals gain `withheld` ─────────────────────────────────────────────────────────────────────────────────
hunk(CONTRACT, '''    catalogGenerationId: artifactCreationCatalogAuthorityFixture.catalog.catalogGenerationId,
    kinds: [JSON.parse(choice)],
  };''', '''    catalogGenerationId: artifactCreationCatalogAuthorityFixture.catalog.catalogGenerationId,
    kinds: [JSON.parse(choice)],
    withheld: [],
  };''')

hunk(OWNER_TEST, '''        kinds: [{ kindId: "s.gis.gismap", schema: "s.gis.gismap", dialect: { artifactKind: "s.gis.gismap", standard: "1", subset: "any" }, label: { en: "GIS Map", de: "GIS-Karte" } }],
      });''', '''        kinds: [{ kindId: "s.gis.gismap", schema: "s.gis.gismap", dialect: { artifactKind: "s.gis.gismap", standard: "1", subset: "any" }, label: { en: "GIS Map", de: "GIS-Karte" } }],
        withheld: [{ kindId: "s.stdio.gif", reason: "ambiguous-editors" }],
      });''')

hunk(OWNER_TEST, '''        { kind: "space-artifact-creation-catalog", clientInstanceId, spaceId: "space-a", catalogGenerationId, kinds: JSON.parse(body).kinds },''', '''        { kind: "space-artifact-creation-catalog", clientInstanceId, spaceId: "space-a", catalogGenerationId, kinds: JSON.parse(body).kinds, withheld: JSON.parse(body).withheld },''')

hunk(ENVELOPE_TEST, '''        kinds: [{ kindId: "s.gis.gismap", schema: "s.gis.gismap", dialect: { artifactKind: "s.gis.gismap", standard: "1", subset: "any" }, label: { en: "GIS Map", de: "GIS-Karte" } }],
      };
      const request: BackboneWorkerRequest''', '''        kinds: [{ kindId: "s.gis.gismap", schema: "s.gis.gismap", dialect: { artifactKind: "s.gis.gismap", standard: "1", subset: "any" }, label: { en: "GIS Map", de: "GIS-Karte" } }],
        withheld: [{ kindId: "s.stdio.gif", reason: "ambiguous-editors" }],
      };
      const request: BackboneWorkerRequest''')

hunk(WGPU_UNIT, '''        kinds: vec![serde_json::from_str(catalog["member"].as_str().expect("member")).expect("catalog member kind")],
    }''', '''        kinds: vec![serde_json::from_str(catalog["member"].as_str().expect("member")).expect("catalog member kind")],
        withheld: Vec::new(),
    }''')

hunk(PROJECTION, '''            { "kindId": "2d.note", "schema": "note.document", "dialect": { "artifactKind": "s.note.note", "standard": "1", "subset": "*" }, "label": { "en": "Note", "de": "Notiz" } }
        ]
    }))''', '''            { "kindId": "2d.note", "schema": "note.document", "dialect": { "artifactKind": "s.note.note", "standard": "1", "subset": "*" }, "label": { "en": "Note", "de": "Notiz" } }
        ],
        "withheld": []
    }))''')

# ── hub (anchored on hold A) ──────────────────────────────────────────────────────────────────────────────────────────
hunk(CAT, r'''/// 🧯 Maximum immutable document-open selections retained by one catalog generation.
pub const TRUSTED_CATALOG_MAX_OPEN_TARGETS: usize = 1024;
''', r'''/// 🧯 Maximum immutable document-open selections retained by one catalog generation.
pub const TRUSTED_CATALOG_MAX_OPEN_TARGETS: usize = 1024;

/// 🧮️ A Space's creation catalog names every kind of a generation in one response: its declared capacity covers the
/// open-target ceiling, since every kind it names is opened by at least one verified editor target.
const _: () = assert!(directory::os_directory::schema::space_artifact_creation::SPACE_ARTIFACT_CREATION_CATALOG_MAX_KINDS >= TRUSTED_CATALOG_MAX_OPEN_TARGETS);
''')

hunk(CAT, r'''    pub(crate) fn artifact_creation_selection(&self, kind_id: &str) -> Option<&VerifiedDocumentOpenSelectionV1> {
        let (hosted, owned): (Vec<_>, Vec<_>) = self.open_targets.iter().filter(|selection| selection.artifact.kind == kind_id && selection.surface.role == DocumentOpenSurfaceRoleV1::Editor).partition(|selection| self.hosted_row(selection).is_some());
        owner_preferred_creation(owned, hosted, |selection| &selection.parent_dialect).one()
    }''', r'''    pub(crate) fn artifact_creation_selection(&self, kind_id: &str) -> Option<&VerifiedDocumentOpenSelectionV1> {
        self.artifact_creation_choice(kind_id).one()
    }

    /// 🌱️ What the creation rule answers for `kind_id` ([`Self::artifact_creation_selection`]): its one creating editor,
    /// `Ambiguous` for several most general editors, `Empty` for a kind no editor opens.
    fn artifact_creation_choice(&self, kind_id: &str) -> MostGeneralDialect<&VerifiedDocumentOpenSelectionV1> {
        let (hosted, owned): (Vec<_>, Vec<_>) = self.open_targets.iter().filter(|selection| selection.artifact.kind == kind_id && selection.surface.role == DocumentOpenSurfaceRoleV1::Editor).partition(|selection| self.hosted_row(selection).is_some());
        owner_preferred_creation(owned, hosted, |selection| &selection.parent_dialect)
    }''')

hunk(CAT, r'''    /// 🗣️ Projects only unambiguous factory-backed choices from retained compiled descriptors. A hosted kind is labelled by its
    /// owner's declaration (a host carries only `{ id, schema, owner }`); a kind no declaration labels is not offered.
    pub(crate) fn artifact_creation_catalog(&self, space_id: &str) -> Option<directory::os_directory::schema::space_artifact_creation::SpaceArtifactCreationCatalogV1> {
        use directory::os_directory::schema::space_artifact_creation::{
            SpaceArtifactCreationCatalogV1, SpaceArtifactCreationDialectV1, SpaceArtifactCreationKindV1, SpaceArtifactCreationLabelV1,
        };
        let kind_ids = self.open_targets.iter().map(|selection| selection.artifact.kind.as_str()).collect::<BTreeSet<_>>();
        let mut kinds = Vec::new();
        for kind_id in kind_ids {
            let Some(selection) = self.artifact_creation_selection(kind_id) else { continue };
            let retained = self.selection_package(selection)?;
            let app = retained.descriptor.manifest.apps.iter().find(|app| app.id == selection.surface.app_id && app.dialect == selection.parent_dialect)?;
            let owner = || self.hosted_kind_owner(selection).and_then(|owner| declared_kind(owner.descriptor.manifest.artifact_kinds.iter().chain(owner.descriptor.manifest.apps.iter().flat_map(|app| app.artifact_kinds.iter())), selection));
            let Some(kind) = declared_kind(app.artifact_kinds.iter().chain(retained.descriptor.manifest.artifact_kinds.iter()), selection).or_else(owner) else { continue };
            kinds.push(SpaceArtifactCreationKindV1 {''', r'''    /// 🗣️ Projects every kind an editor of this generation opens from retained compiled descriptors: the offered choices, and
    /// each kind that is not offered NAMED with its typed reason — several most general editors (two standards), no
    /// declaration labelling it (a hosted kind is labelled by its owner's declaration; a host carries only `{ id, schema,
    /// owner }`), or a row that cannot be presented (a label over 128 characters). One such kind never withholds the others.
    pub(crate) fn artifact_creation_catalog(&self, space_id: &str) -> Option<directory::os_directory::schema::space_artifact_creation::SpaceArtifactCreationCatalogV1> {
        use directory::os_directory::schema::space_artifact_creation::{
            SpaceArtifactCreationCatalogV1, SpaceArtifactCreationDialectV1, SpaceArtifactCreationKindV1, SpaceArtifactCreationLabelV1, SpaceArtifactCreationWithheldKindV1, SpaceArtifactCreationWithheldReasonV1,
        };
        let kind_ids = self.open_targets.iter().map(|selection| selection.artifact.kind.as_str()).collect::<BTreeSet<_>>();
        let (mut kinds, mut withheld) = (Vec::new(), Vec::new());
        let withhold = |kind_id: &str, reason| SpaceArtifactCreationWithheldKindV1 { kind_id: kind_id.into(), reason };
        for kind_id in kind_ids {
            let selection = match self.artifact_creation_choice(kind_id) {
                MostGeneralDialect::One(selection) => selection,
                MostGeneralDialect::Ambiguous => {
                    withheld.push(withhold(kind_id, SpaceArtifactCreationWithheldReasonV1::AmbiguousEditors));
                    continue;
                }
                MostGeneralDialect::Empty => continue,
            };
            let retained = self.selection_package(selection)?;
            let app = retained.descriptor.manifest.apps.iter().find(|app| app.id == selection.surface.app_id && app.dialect == selection.parent_dialect)?;
            let owner = || self.hosted_kind_owner(selection).and_then(|owner| declared_kind(owner.descriptor.manifest.artifact_kinds.iter().chain(owner.descriptor.manifest.apps.iter().flat_map(|app| app.artifact_kinds.iter())), selection));
            let Some(kind) = declared_kind(app.artifact_kinds.iter().chain(retained.descriptor.manifest.artifact_kinds.iter()), selection).or_else(owner) else {
                withheld.push(withhold(kind_id, SpaceArtifactCreationWithheldReasonV1::Unlabelled));
                continue;
            };
            let choice = SpaceArtifactCreationKindV1 {''')

hunk(CAT, r'''                    de: kind.label.resolve(semio_framework::Terminology::Native, semio_framework::Locale::De).to_string(),
                },
            });
        }
        kinds.sort_by(|left, right| left.kind_id.cmp(&right.kind_id));
        let catalog = SpaceArtifactCreationCatalogV1 { schema: "semio.hub.space-artifact-creation-catalog/v1".into(), space_id: space_id.into(), catalog_generation_id: self.generation_id.clone(), kinds };''', r'''                    de: kind.label.resolve(semio_framework::Terminology::Native, semio_framework::Locale::De).to_string(),
                },
            };
            if choice.validate() {
                kinds.push(choice);
            } else {
                withheld.push(withhold(kind_id, SpaceArtifactCreationWithheldReasonV1::Unpresentable));
            }
        }
        kinds.sort_by(|left, right| left.kind_id.cmp(&right.kind_id));
        withheld.sort_by(|left, right| left.kind_id.cmp(&right.kind_id));
        let catalog = SpaceArtifactCreationCatalogV1 { schema: "semio.hub.space-artifact-creation-catalog/v1".into(), space_id: space_id.into(), catalog_generation_id: self.generation_id.clone(), kinds, withheld };''')

hunk(CAT_LAW, r'''fn hosting_descriptor_bytes(component_sha256: &str, schema: &str) -> Vec<u8> {''', r'''fn hosting_descriptor_bytes(component_sha256: &str, schema: &str) -> Vec<u8> {
    hosting_descriptor_bytes_with(component_sha256, schema, "1", "strict")
}

/// 🏠️ [`hosting_descriptor_bytes`] with the second editor's `standard` and `subset` chosen: `2` / `*` makes two most general
/// editors of two standards.
fn hosting_descriptor_bytes_with(component_sha256: &str, schema: &str, standard: &str, subset: &str) -> Vec<u8> {''')

hunk(CAT_LAW, r'''    let mut strict = manifest["apps"][0].clone();
    strict["id"] = "s.fixture.document@1/strict#editor".into();
    strict["controllerId"] = "fixture-editor-strict".into();
    strict["dialect"]["subset"] = "strict".into();''', r'''    let mut strict = manifest["apps"][0].clone();
    strict["id"] = format!("s.fixture.document@{standard}/{subset}#editor").into();
    strict["controllerId"] = "fixture-editor-strict".into();
    strict["dialect"]["standard"] = standard.into();
    strict["dialect"]["subset"] = subset.into();''')

hunk(CAT_LAW, r'''        assert_eq!((hosting.rows, hosting.rows_pinned), (1, u64::from(linked)), "the hosted identity is a row of the host, pinned only when linked");
    }
}''', r'''        assert_eq!((hosting.rows, hosting.rows_pinned), (1, u64::from(linked)), "the hosted identity is a row of the host, pinned only when linked");
    }
}

/// 🚫️ LAW: the Space creation catalog names every kind an editor of the generation opens — offered when one most general
/// editor creates it and its owner's declaration labels it presentably, otherwise WITHHELD with its typed reason: two most
/// general editors of two standards (`ambiguous-editors`), a label over 128 characters (`unpresentable`). Nothing is dropped
/// silently and one withheld kind never withholds the catalog (ticket 26/09/23 H14: the first hosted kind turned the whole
/// catalog into 409, and multi-standard kinds vanished without a word). The fixture's schema is made an identity, as every
/// shipped schema is, because a creation row names its schema.
#[tokio::test]
async fn the_creation_catalog_offers_or_names_every_kind_an_editor_opens_with_its_reason() {
    use directory::os_directory::schema::space_artifact_creation::{SpaceArtifactCreationWithheldKindV1, SpaceArtifactCreationWithheldReasonV1};
    let component_sha256 = fixture_json()["componentSha256"].as_str().expect("component sha256").to_owned();
    let hosting = |standard: &str, subset: &str, owner_label: Option<String>| {
        let mut fixture = prepared_fixture();
        let presentable = fixture.schema.replace('@', ".v");
        fixture.bundle = serde_json::from_str(&fixture.bundle.to_string().replace(&fixture.schema, &presentable)).expect("fixture bundle with an identity schema");
        fixture.schema = presentable;
        let schema = fixture.schema.clone();
        let codecs = fixture.bundle["packages"][0]["nativeCodecs"].take();
        fixture.bundle["packages"][0]["nativeCodecs"] = serde_json::json!([]);
        fixture.bundle["packages"][1]["nativeCodecs"] = codecs;
        let mut second = fixture.bundle["packages"][0]["openTargets"][0].clone();
        second["surfaceId"] = format!("s.fixture.document@{standard}/{subset}#editor").into();
        second["appId"] = second["surfaceId"].clone();
        second["parentDialect"]["standard"] = standard.into();
        second["parentDialect"]["subset"] = subset.into();
        fixture.bundle["packages"][0]["openTargets"].as_array_mut().expect("host targets").insert(0, second.clone());
        let mut selected = fixture.bundle["profiles"][0]["openTargets"][0].clone();
        selected["target"] = second;
        fixture.bundle["profiles"][0]["openTargets"].as_array_mut().expect("profile targets").insert(0, selected);
        let mut owner = descriptor_json("fixture.base", "semio:fixture-base", "1.0.0", &component_sha256, Some(&schema), None);
        if let Some(label) = owner_label {
            owner["manifest"]["artifactKinds"][0]["label"]["native"]["en"] = label.into();
        }
        fixture.replace_descriptor(1, encode_descriptor_json(owner));
        fixture.replace_descriptor(0, hosting_descriptor_bytes_with(&component_sha256, &schema, standard, subset));
        fixture
    };
    let named = |withheld: SpaceArtifactCreationWithheldReasonV1| vec![SpaceArtifactCreationWithheldKindV1 { kind_id: "s.fixture.document".into(), reason: withheld }];
    for (standard, subset, owner_label, offered, withheld) in [
        ("1", "strict", None, true, Vec::new()),
        ("2", "*", None, false, named(SpaceArtifactCreationWithheldReasonV1::AmbiguousEditors)),
        ("1", "strict", Some("x".repeat(129)), false, named(SpaceArtifactCreationWithheldReasonV1::Unpresentable)),
    ] {
        let fixture = hosting(standard, subset, owner_label);
        let binding = NativeCodecBinding::new("fixture.base", "semio:fixture-base", "s.fixture.document", fixture_codec(&fixture.schema, [0x11; 32]));
        let catalog = load_fixture(&fixture, &[binding], &TestControl::new().context()).await.expect("hosting catalog loads");
        let creation = catalog.artifact_creation_catalog("space").expect("a catalog naming a kind is served");
        assert_eq!(creation.kinds.iter().map(|kind| kind.kind_id.as_str()).collect::<Vec<_>>(), if offered { vec!["s.fixture.document"] } else { Vec::new() }, "{standard}/{subset}");
        assert_eq!(creation.withheld, withheld, "{standard}/{subset}");
        assert!(creation.canonical_json().is_some(), "{standard}/{subset}: the catalog is served, never refused for one withheld kind");
    }
}''')

hunk(LAW_RS, r'''        assert_eq!((own, independent), (accepted, accepted), "{}", row["id"]);
    }
}
''', r'''        assert_eq!((own, independent), (accepted, accepted), "{}", row["id"]);
    }
}

/// 🧮️ LAW: a creation catalog at its declared capacity whose every row is at its maximum validates, round-trips and serializes
/// to exactly one byte under the derived byte ceiling (the derivation is tight; serde_json agrees on the length); one more
/// named kind — offered or withheld — is refused; and the capacity covers the trusted catalog's open-target ceiling, so a
/// verified catalog is never cut off (t6 carries ≈ 90 creatable kinds against the former 64, ticket 26/09/23 H14).
#[test]
fn a_creation_catalog_at_capacity_with_maximal_rows_fits_its_derived_bytes_and_one_more_kind_is_refused() {
    use directory::os_directory::schema::space_artifact_creation::{
        SpaceArtifactCreationDialectV1, SpaceArtifactCreationKindV1, SpaceArtifactCreationLabelV1, SpaceArtifactCreationWithheldKindV1, SpaceArtifactCreationWithheldReasonV1, SPACE_ARTIFACT_CREATION_CATALOG_MAX_BYTES,
        SPACE_ARTIFACT_CREATION_CATALOG_MAX_KINDS,
    };
    let identity = |seed: &str| format!("{seed}{}", "x".repeat(256 - seed.len()));
    let label = "\u{1D11E}".repeat(128);
    let row = |index: usize| SpaceArtifactCreationKindV1 {
        kind_id: format!("{index:0>256}"),
        schema: identity("schema"),
        dialect: SpaceArtifactCreationDialectV1 { artifact_kind: identity("kind"), standard: identity("standard"), subset: identity("subset") },
        label: SpaceArtifactCreationLabelV1 { en: label.clone(), de: label.clone() },
    };
    let catalog = |count: usize| SpaceArtifactCreationCatalogV1 { schema: "semio.hub.space-artifact-creation-catalog/v1".into(), space_id: identity("space"), catalog_generation_id: "ab".repeat(32), kinds: (0..count).map(row).collect(), withheld: Vec::new() };
    let full = catalog(SPACE_ARTIFACT_CREATION_CATALOG_MAX_KINDS);
    let canonical = full.canonical_json().expect("a full catalog of maximal rows is served");
    assert_eq!(canonical.len() + 1, SPACE_ARTIFACT_CREATION_CATALOG_MAX_BYTES, "the derived ceiling is exactly the worst case");
    assert_eq!(serde_json::to_string(&full).expect("serde_json oracle").len(), canonical.len());
    assert_eq!(SpaceArtifactCreationCatalogV1::parse_canonical_json(&canonical).as_ref(), Some(&full));
    let over = catalog(SPACE_ARTIFACT_CREATION_CATALOG_MAX_KINDS + 1);
    assert!(!over.validate() && over.canonical_json().is_none(), "one offered kind over the capacity is refused");
    let mut withheld_over = full.clone();
    withheld_over.withheld.push(SpaceArtifactCreationWithheldKindV1 { kind_id: "z".repeat(256), reason: SpaceArtifactCreationWithheldReasonV1::AmbiguousEditors });
    assert!(!withheld_over.validate(), "one withheld kind over the capacity is refused");
    assert!(SPACE_ARTIFACT_CREATION_CATALOG_MAX_KINDS >= crate::artifact_authority::trusted_catalog::TRUSTED_CATALOG_MAX_OPEN_TARGETS);
}
''')

hunk(LAW_TS, '''import { parseSpaceArtifactCreationStatusJsonV1 } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts";''', '''import {
  SPACE_ARTIFACT_CREATION_CATALOG_MAX_BYTES,
  SPACE_ARTIFACT_CREATION_CATALOG_MAX_KINDS,
  SPACE_ARTIFACT_CREATION_CATALOG_SCHEMA_V1,
  parseSpaceArtifactCreationCatalogJsonV1,
  parseSpaceArtifactCreationStatusJsonV1,
} from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts";''')

hunk(LAW_TS, '''      expect(parsed, row.id).toBe(row.accepted);
    }
  });
});
''', '''      expect(parsed, row.id).toBe(row.accepted);
    }
  });

  it("Ajv and the TypeScript twin judge every catalog row, offered and withheld, as the fixture does", () => {
    const ajv = new Ajv({ allErrors: true, strict: false });
    ajv.addSchema({ ...module, $id: "urn:semio:directory" });
    const validate = ajv.getSchema("urn:semio:directory#/$defs/SpaceArtifactCreationCatalog")!;
    expect(fixture.catalogs.some((row: any) => row.accepted && row.value.withheld.length > 0 && row.value.kinds.length === 0)).toBe(true);
    for (const row of fixture.catalogs) {
      expect(validate(row.value), `${row.id}: ${JSON.stringify(validate.errors)}`).toBe(row.schemaValid ?? row.accepted);
      let parsed = true;
      try {
        parseSpaceArtifactCreationCatalogJsonV1(JSON.stringify(row.value));
      } catch {
        parsed = false;
      }
      expect(parsed, row.id).toBe(row.accepted);
    }
  });

  it("Ajv and the TypeScript twin accept a catalog at its capacity with maximal rows, exactly one byte under the derived ceiling, and refuse one kind more", () => {
    const ajv = new Ajv({ allErrors: true, strict: false });
    ajv.addSchema({ ...module, $id: "urn:semio:directory" });
    const validate = ajv.getSchema("urn:semio:directory#/$defs/SpaceArtifactCreationCatalog")!;
    const identity = (seed: string) => seed + "x".repeat(256 - seed.length);
    const label = "\\u{1D11E}".repeat(128);
    const row = (index: number) => ({ kindId: String(index).padStart(256, "0"), schema: identity("schema"), dialect: { artifactKind: identity("kind"), standard: identity("standard"), subset: identity("subset") }, label: { en: label, de: label } });
    const catalog = (count: number) => ({ schema: SPACE_ARTIFACT_CREATION_CATALOG_SCHEMA_V1, spaceId: identity("space"), catalogGenerationId: "ab".repeat(32), kinds: Array.from({ length: count }, (_, index) => row(index)), withheld: [] });
    const full = catalog(SPACE_ARTIFACT_CREATION_CATALOG_MAX_KINDS);
    const source = JSON.stringify(full);
    expect(validate(full), JSON.stringify(validate.errors)).toBe(true);
    expect(new TextEncoder().encode(source).byteLength + 1).toBe(SPACE_ARTIFACT_CREATION_CATALOG_MAX_BYTES);
    expect(parseSpaceArtifactCreationCatalogJsonV1(source).kinds).toHaveLength(SPACE_ARTIFACT_CREATION_CATALOG_MAX_KINDS);
    const over = catalog(SPACE_ARTIFACT_CREATION_CATALOG_MAX_KINDS + 1);
    expect(validate(over)).toBe(false);
    expect(() => parseSpaceArtifactCreationCatalogJsonV1(JSON.stringify(over))).toThrow();
    const withheldOver = { ...full, withheld: [{ kindId: "z".repeat(256), reason: "ambiguous-editors" }] };
    expect(() => parseSpaceArtifactCreationCatalogJsonV1(JSON.stringify(withheldOver))).toThrow();
  });
});
''')

out = pathlib.Path(__file__).with_name('h14-catalog-bound.json')
out.write_text(json.dumps(H, ensure_ascii=False, indent=1) + '\n')
print(len(H), 'hunks,', len({h['file'] for h in H}), 'files ->', out.name)
