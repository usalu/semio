# Current Integration Diagnostics

The previous source and native matrices reached terminal RED. These are observed diagnostics, not current-source certification: concurrent mutation refactoring continued overnight. Fresh targeted checks will distinguish already-repaired diagnostics from required integration work.

## iso16757-current.log

```text
error: DslRecord only supports structs
   --> 🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/📦️packages/🦀️rust/../../🦀️.rs:97:1
    |
 97 | / /// 🔢️ Typed catalogue value.
 98 | | #[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
 99 | | #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
100 | | #[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase"))]
...   |
114 | |     List { items: Vec<CatalogueValue> },
115 | | }
    | |_^

warning: `semio-s-artifact-norm-contract` (lib) generated 18 warnings (run `cargo fix --lib -p semio-s-artifact-norm-contract` to apply 18 suggestions)
[cargo:build] running elapsedMs=880173
```

## complementary-source-current.log

```text
error: expect(received).toBe(expected)

Expected: true
Received: false

      at <anonymous> (/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts:38:67)
✗ fem2d handwritten semantic SQLite capability is owned by its snapshot [1.72ms]
✓ fem2d complete literal state and empty holes through independent SQLite edits [92.44ms]
✓ fem2d every native IEEE word and complete unsigned count widths [273.23ms]
✓ fem2d malformed relationship variants, storage types, words and schema are refused [46.39ms]
✓ fem2d genuine borrowed UTF frontiers and cumulative domain bounds [18.40ms]
✓ fem2d independent SQLite DDL owns relationships and exact scalar columns [13.31ms]
✓ fem2d canonical snapshot artifact and diff exact word 0000000000000000 [43.68ms]
✓ fem2d canonical snapshot artifact and diff exact word 8000000000000000 [37.88ms]
```

## complementary-source-current.log

```text
error: expect(received).toBe(expected)

Expected: true
Received: false

      at <anonymous> (/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts:36:67)
✗ fem3d handwritten semantic SQLite capability is owned by its snapshot [1.00ms]
✓ fem3d complete independent file edits preserve literal references, empty holes and UTF-8 map order [140.89ms]
✓ fem3d complete scalar word 0000000000000000 [111.08ms]
✓ fem3d complete scalar word 8000000000000000 [47.54ms]
✓ fem3d complete scalar word 7ff0000000000000 [54.82ms]
✓ fem3d complete scalar word fff0000000000000 [14.81ms]
✓ fem3d complete scalar word 7ff8000000000042 [32.37ms]
✓ fem3d complete scalar word 7ff0000000000001 [24.44ms]
```

## complementary-source-current.log

```text
error: expect(received).toBe(expected)

Expected: "function"
Received: "undefined"

      at <anonymous> (/Users/ueli/Documents/semio/✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts:78:100)
✗ Forms actual snapshot module owns relational export and import [1.48ms]
✓ Forms actual consumers accept canonical tagged Boolean and full integer answers [0.59ms]
✓ Framework intrinsic owner preserves unsigned words, NaN identity and duplicate ordered members [7.48ms]
13 | }
14 | function specimen():any{
15 |  const q=fixture.question,values=fixture.values.map(portableValue),child=(value:typeof fixture.structure)=>({childId:value.childId,target:{artifactId:value.artifactId,dialect:{artifactKind:value.artifactKind,standard:value.standard,subset:value.subset}}});
16 |  return{schema:fixture.schema,id:fixture.id,version:fixture.version,title:fixture.title,definition:{steps:[{...fixture.step,blocks:[{id:q.id,label:q.label,kind:q.kind,description:q.description,required:q.required,placeholder:q.placeholder,default:{kind:"array",items:values},min:binary64Value({bits:BigInt("0x"+q.minimumBits)}),max:binary64Value({bits:BigInt("0x"+q.maximumBits)}),step:binary64Value({bits:BigInt("0x"+q.incrementBits)}),unit:q.unit,text:q.text,options:q.options,fields:q.fields.map(f=>({key:f.key,...("label"in f?{label:f.label}:{}),value:binary64Value({bits:BigInt("0x"+f.valueBits)})})),schema:q.questionSchema,src:q.src,accept:q.accept,exampleId:q.exampleId,params:portableValue(fixture.values[8]),condition:{kind:"eq",left:{kind:"const",value:portableValue(fixture.values[2])},right:{kind:"or",items:[{kind:"var",name:"answer"},{kind:"truthy",expr:{kind:"and",items:[]}}]}}}]}]},responses:[{id:fixture.response.id,submittedAt:Number(fixture.response.submittedAt),definitionVersion:fixture.response.defin | ... truncated 
17 | }
```

## complementary-source-current.log

```text
error: expect(received).toBe(expected)

Expected: true
Received: false

      at <anonymous> (/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts:28:133)
✗ CAD public persisted facade exposes both semantic directions [0.21ms]
18 | const ports=owner as unknown as Ports;
19 | function snapshot():Snapshot{
20 |  const value=structuredClone(fixture.snapshot);
21 |  return{...value,referencesByModelDefinitionId:Object.fromEntries(Object.entries(value.referencesByModelDefinitionId).map(([key,rows])=>[key,rows.map(row=>({...row,origin:row.origin.map(binary64)as Reference["origin"],orientation:row.orientation===null?null:row.orientation.map(binary64)as Reference["orientation"],scale:row.scale===null?null:binary64(row.scale),widthWorld:binary64(row.widthWorld),opacity:row.opacity===null?null:binary64(row.opacity)}))]))};
22 | }
23 | const bytes=async(value:Snapshot)=>exportSqliteDatabase(await ports.cadSnapshotToSqliteDatabase(value));
                                                                         ^
```

## complementary-source-current.log

```text
error: expect(received).toBe(expected)

Expected: true
Received: false

      at <anonymous> (/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts:29:152)
✗ Architect public persisted facade exposes both complete semantic directions [0.19ms]
25 | test("Architect canonical full-width unsigned64 entity admission retains exact words and refuses coercion",async()=>{const parsers:Record<string,(value:unknown)=>unknown>={AnalysisRecord:artifact.parseAnalysisRecord,SearchFilter:artifact.parseSearchFilter,TemplateRecord:artifact.parseTemplateRecord,KnowledgeRecord:artifact.parseKnowledgeRecord};const fixtures=laws.unsigned64Fixtures as Record<string,string[]>;for(const[entity,field]of laws.unsigned64Fields){const[path,slot]=fixtures[entity]!;const source=await Bun.file(new URL("../../../.."+path,import.meta.url)).json() as Record<string,Record<string,unknown>[]>;const original=(programArtifactFromJson(source) as unknown as Record<string,Record<string,unknown>[]>)[slot!]![0]!;for(const raw of laws.unsigned64Corpus.valid){const word=BigInt(raw);expect(word.toString()).toBe(raw);const expected={...original,[field!]:word};expect(parsers[entity]!(expected)).toEqual(expected)}for(const value of [...laws.unsigned64Corpus.invalid.map(BigInt),0,1,0.5,"0",true,NaN,Infi | ... truncated 
26 | test("Architect independent schema admits the persisted parent fixture",()=>{expect(independentSchema(snapshot)).toBe(true);const expected=programArtifactFromJson(snapshot);expect(owner.parseProgramSnapshot(expected) as unknown).toEqual(expected)});
27 | test("Architect independent schema admits literal independent child identities",()=>{const input=structuredClone(snapshot);input.knowledge.childId="local 世界\0";input.knowledge.target.artifactId="foreign !@";input.benchmarks.childId="";input.benchmarks.target.artifactId="same";expect(independentSchema(input)).toBe(true);const expected=programArtifactFromJson(input);expect(owner.parseProgramSnapshot(expected) as unknown).toEqual(expected)});
28 | test("Architect neutral identity corpus is independently parsed without normalizing literal domains",()=>{expect(JSON.parse(JSON.stringify(laws.idTexts))).toEqual(laws.idTexts);expect(laws.control.largeCharacters).toBeGreaterThan(65536)});
29 | test("Architect public persisted facade exposes both complete semantic directions",()=>{expect(Object.hasOwn(owner,"programSnapshotToSqliteDatabase")).toBe(true);expect(Object.hasOwn(owner,"programSnapshotFromSqliteDatabase")).toBe(true)});
30 | async function completeFixture(){const input=structuredClone(snapshot) as Record<string,unknown>;for(const[key,folder,caseName]of laws.registerFixtures){const source=await Bun.file(new URL("../../../../🧫️fixtures/🧬️mutations"+folder+"/"+caseName+"/📸️snapshot/➡️after/🔣️.json",import.meta.url)).json();expect(source[key!]).toHaveLength(1);input[key!]=source[key!]}return programArtifactFromJson(input)}
                                                                                                                                                                                                                                                                                                              ^
```

## complementary-source-current.log

```text
error: expect(received).toBe(expected)

Expected: true
Received: false

      at <anonymous> (/Users/ueli/Documents/semio/✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/📋️contract/🟦️.ts:8:144)
✗ DAG actual persisted Snapshot owns complete parent SQLite capability [0.17ms]
✓ DAG canonical Source keeps literal child empty-address [0.24ms]
✓ DAG canonical Source keeps literal child independent-unresolved-alias [0.02ms]
✓ DAG canonical Source keeps literal child unrestricted-literal
✓ DAG two handwritten tables preserve all literal child fields independently [2.64ms]
✓ DAG canonical domain rejects wrong child dialect without constraining literal IDs [0.09ms]
✓ DAG actual public physical SQLite roundtrip and independent edited reference empty-address [12.48ms]
✓ DAG actual public physical SQLite roundtrip and independent edited reference independent-unresolved-alias [2.13ms]
```

## complementary-source-current.log

```text
error: expect(received).toBe(expected)

Expected: "function"
Received: "undefined"

      at <anonymous> (/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts:32:23)
✗ closed genuine JSON transport preserves every declared binary64 word and rejects malformed carriers [0.42ms]
✓ typed Semio Kit catalog dialect is admitted across literal child identities and SQL directions [24.60ms]
✓ closed all-cell semantic corpus is independently measured in every actual SQL storage class [53.01ms]

1 tests failed:
✗ closed genuine JSON transport preserves every declared binary64 word and rejects malformed carriers [0.42ms]

 64 pass
```

## complementary-source-current.log

```text
error: expect(received).toBe(expected)

Expected: true
Received: false

      at <anonymous> (/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts:14:123)
✗ VCS facade declares the full semantic SQLite owner [1.45ms]
✓ VCS handwritten schema is independently queryable without native codec knowledge [5.59ms]
15 | test("VCS handwritten schema is independently queryable without native codec knowledge",async()=>{
16 |  const sql=await Bun.file(new URL("../🗄️.sql",import.meta.url)).text(),db=new Database(":memory:",{safeIntegers:true});
17 |  try{db.run(sql);db.run("INSERT INTO vcs_document VALUES (1,?,?,?,?,?)",[fixture.snapshot.schema,fixture.snapshot.title,-9223372036854775808n,fixture.snapshot.notes,fixture.snapshot.status]);fixture.snapshot.tags.forEach((tag,index)=>db.run("INSERT INTO vcs_tag VALUES (?,?,?,?)",[BigInt(index+1),1n,BigInt(index),tag]));expect(db.query("SELECT counter FROM vcs_document").get()).toEqual({counter:-9223372036854775808n});expect(db.query("SELECT value FROM vcs_tag JOIN vcs_document ON vcs_document.id=vcs_tag.document_id ORDER BY ordinal").all()).toEqual(fixture.snapshot.tags.map(value=>({value})));expect(db.query("PRAGMA foreign_key_check").all()).toEqual([])}finally{db.close()}
18 | });
19 | test("VCS owned files preserve full fields, tag order, duplicates, empty tags and Unicode",async()=>{
20 |  const expected=snapshot(),database=await own.vcsSnapshotToSqliteDatabase(expected);expect(own.VCS_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../🗄️.sql",import.meta.url)).text());expect(database.tables.length).toBe(2);
```

## complementary-source-current.log

```text
error: expect(received).toBe(expected)

Expected: true
Received: false

      at <anonymous> (/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts:49:73)
✗ Playground actual Snapshot owns both semantic SQLite directions [0.24ms]
✓ Playground literal persisted schema Default Marker [0.12ms]
✓ Playground literal persisted schema Present Empty Marker
✓ Playground literal persisted schema Literal Unicode Marker
✓ Playground literal persisted schema Literal JSON Looking Marker
✓ Playground closed persisted shape follows the independent JSON schema [0.13ms]
✓ Playground hand-authored SQL is independently queryable without a carrier [2.24ms]
77 |   } finally { db.close(); }
```

## complementary-source-current.log

```text
error: expect(received).toBe(expected)

Expected: true
Received: false

      at <anonymous> (/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts:398:56)
✗ Rewriting actual Semio child public JSON codec preserves all words and intrinsic families [1.18ms]
415 |  const invalid=structuredClone(vector.childSnapshot);invalid.nodes[0].width={bits:"INVALID"};expect(()=>api.parseSemioGraphJsonValue(invalid)).toThrow();invalid.nodes[0].width={bits:"0000000000000000",extra:true};expect(()=>api.parseSemioGraphJsonValue(invalid)).toThrow();
416 | });
417 |
418 | test("Rewriting retained child capture contract preserves exact publication and release authority",async()=>{
419 |  const base=new URL("../../../../🧬️schema/📸️snapshot/🧫️fixtures/🪆️child/👁️capture",import.meta.url);
420 |  const contract=await Bun.file(new URL("🔣️.json",base)).json();
                                                              ^
```

## complementary-native-current.log

```text
error[E0432]: unresolved import `set_playback_clock`
 --> 🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🚪️io/💾️binary/🧬️mutations/🦀️.rs:8:5
  |
8 | use set_playback_clock::SetPlaybackClock;
  |     ^^^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `set_playback_clock`
  |
  = help: if you wanted to use a crate named `set_playback_clock`, use `cargo add set_playback_clock` to add it to your `Cargo.toml`

error[E0432]: unresolved import `set_playback_clock`
 --> 🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🚪️io/📝️text/🧬️mutations/🦀️.rs:8:5
  |
8 | use set_playback_clock::SetPlaybackClock;
  |     ^^^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `set_playback_clock`
  |
```

## complementary-native-current.log

```text
error[E0432]: unresolved import `set_playback_clock`
 --> 🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🚪️io/📝️text/🧬️mutations/🦀️.rs:8:5
  |
8 | use set_playback_clock::SetPlaybackClock;
  |     ^^^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `set_playback_clock`
  |
  = help: if you wanted to use a crate named `set_playback_clock`, use `cargo add set_playback_clock` to add it to your `Cargo.toml`

error[E0432]: unresolved import `set_playback_clock`
  --> 🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🚪️io/📝️text/🧬️mutations/🦀️.rs:26:5
   |
26 | use set_playback_clock::SetPlaybackClock;
   |     ^^^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `set_playback_clock`
   |
```

## complementary-native-current.log

```text
error[E0432]: unresolved import `set_playback_clock`
  --> 🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🚪️io/📝️text/🧬️mutations/🦀️.rs:26:5
   |
26 | use set_playback_clock::SetPlaybackClock;
   |     ^^^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `set_playback_clock`
   |
   = help: if you wanted to use a crate named `set_playback_clock`, use `cargo add set_playback_clock` to add it to your `Cargo.toml`

error[E0425]: cannot find value `COMPONENT_GRAMMAR_SEMIO` in module `standards::v1::subsets::any::schema::diff`
   --> 🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../../🦀️.rs:457:78
    |
457 |                     grammar: Some(standards::v1::subsets::any::schema::diff::COMPONENT_GRAMMAR_SEMIO),
    |                                                                              ^^^^^^^^^^^^^^^^^^^^^^^ not found in `standards::v1::subsets::any::schema::diff`
    |
```

## complementary-native-current.log

```text
error[E0425]: cannot find value `COMPONENT_GRAMMAR_SEMIO` in module `standards::v1::subsets::any::schema::diff`
   --> 🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../../🦀️.rs:457:78
    |
457 |                     grammar: Some(standards::v1::subsets::any::schema::diff::COMPONENT_GRAMMAR_SEMIO),
    |                                                                              ^^^^^^^^^^^^^^^^^^^^^^^ not found in `standards::v1::subsets::any::schema::diff`
    |
help: consider importing one of these constants
    |
 44 + use crate::standards::v1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_SEMIO;
    |
 44 + use crate::standards::v1::subsets::any::io::text::inferences::COMPONENT_GRAMMAR_SEMIO;
    |
 44 + use crate::standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO;
    |
```

## complementary-native-current.log

```text
error[E0425]: cannot find value `COMPONENT_GRAMMAR_PATH` in module `standards::v1::subsets::any::schema::diff`
   --> 🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../../🦀️.rs:458:83
    |
458 |                     grammar_path: Some(standards::v1::subsets::any::schema::diff::COMPONENT_GRAMMAR_PATH),
    |                                                                                   ^^^^^^^^^^^^^^^^^^^^^^ not found in `standards::v1::subsets::any::schema::diff`
    |
help: consider importing one of these constants
    |
 44 + use crate::standards::v1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_PATH;
    |
 44 + use crate::standards::v1::subsets::any::io::text::inferences::COMPONENT_GRAMMAR_PATH;
    |
 44 + use crate::standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_PATH;
    |
```

## complementary-native-current.log

```text
error[E0425]: cannot find function `empty_fem2d_snapshot` in module `schema`
  --> 🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:36:165
   |
36 | ...ope(crate::FEM_2D_SCHEMA, "fem2d", schema::empty_fem2d_snapshot(), None), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()))).ex...
   |                                               ^^^^^^^^^^^^^^^^^^^^ not found in `schema`
   |
help: consider importing this function through its public re-export
   |
 1 + use crate::standards::v1::subsets::any::io::text::snapshot::empty_fem2d_snapshot;
   |
help: if you import `empty_fem2d_snapshot`, refer to it directly
   |
36 -     let mut store = ::semio_framework_async::poll::resolve_ready(schema::mutations::Fem2dStore::new(create_document_envelope(crate::FEM_2D_SCHEMA, "fem2d", schema::empty_fem2d_snapshot(), None), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()))).expect("valid store");
36 +     let mut store = ::semio_framework_async::poll::resolve_ready(schema::mutations::Fem2dStore::new(create_document_envelope(crate::FEM_2D_SCHEMA, "fem2d", empty_fem2d_snapshot(), None), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()))).expect("valid store");
```

## complementary-native-current.log

```text
error[E0425]: cannot find value `PLANAR_DOFS` in this scope
   --> 🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📝️text/🧬️mutations/🦀️.rs:150:27
    |
150 |         for (at, name) in PLANAR_DOFS.iter().enumerate() {
    |                           ^^^^^^^^^^^ not found in this scope
    |
note: constant `crate::standards::v1::subsets::any::schema::mutations::component::PLANAR_DOFS` exists but is inaccessible
   --> 🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/🦀️.rs:477:1
    |
477 | const PLANAR_DOFS: [&str; 3] = ["Tx", "Ty", "Rz"];
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not accessible

warning: unused imports: `Buildable` and `HasBase`
  --> 🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../../../../⚙️engine/🖥️app-surface/🦀️.rs:18:35
```

## complementary-native-current.log

```text
error[E0599]: no method named `__dsl_to_record` found for reference `&standards::v1::subsets::any::schema::diff::component::Fem2dDiff` in the current scope
  --> 🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📝️text/🔺️diff/🦀️.rs:11:1
   |
11 | semio_framework_os_kernel::diff_text!(crate::standards::v1::subsets::any::schema::diff::Fem2dDiff);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&standards::v1::subsets::any::schema::diff::component::Fem2dDiff`
   |
   = note: this error originates in the macro `semio_framework_os_kernel::diff_text` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0599]: no associated function or constant named `__dsl_spec` found for struct `standards::v1::subsets::any::schema::diff::component::Fem2dDiff` in the current scope
  --> 🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📝️text/🔺️diff/🦀️.rs:11:1
   |
11 | semio_framework_os_kernel::diff_text!(crate::standards::v1::subsets::any::schema::diff::Fem2dDiff);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ associated function or constant not found in `standards::v1::subsets::any::schema::diff::component::Fem2dDiff`
   |
```

## complementary-native-current.log

```text
error[E0599]: no associated function or constant named `__dsl_spec` found for struct `standards::v1::subsets::any::schema::diff::component::Fem2dDiff` in the current scope
  --> 🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📝️text/🔺️diff/🦀️.rs:11:1
   |
11 | semio_framework_os_kernel::diff_text!(crate::standards::v1::subsets::any::schema::diff::Fem2dDiff);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ associated function or constant not found in `standards::v1::subsets::any::schema::diff::component::Fem2dDiff`
   |
  ::: 🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🔺️diff/🦀️.rs:12:1
   |
12 | pub struct Fem2dDiff {
   | -------------------- associated function or constant `__dsl_spec` not found for this struct
   |
   = note: this error originates in the macro `semio_framework_os_kernel::diff_text` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0599]: no associated function or constant named `__dsl_from_record` found for struct `standards::v1::subsets::any::schema::diff::component::Fem2dDiff` in the current scope
```

## complementary-native-current.log

```text
error[E0599]: no associated function or constant named `__dsl_from_record` found for struct `standards::v1::subsets::any::schema::diff::component::Fem2dDiff` in the current scope
  --> 🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📝️text/🔺️diff/🦀️.rs:11:1
   |
11 | semio_framework_os_kernel::diff_text!(crate::standards::v1::subsets::any::schema::diff::Fem2dDiff);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ associated function or constant not found in `standards::v1::subsets::any::schema::diff::component::Fem2dDiff`
   |
  ::: 🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🔺️diff/🦀️.rs:12:1
   |
12 | pub struct Fem2dDiff {
   | -------------------- associated function or constant `__dsl_from_record` not found for this struct
   |
   = note: this error originates in the macro `semio_framework_os_kernel::diff_text` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0599]: no associated function or constant named `__dsl_spec` found for struct `standards::v1::subsets::any::schema::diff::component::Fem2dDiff` in the current scope
```

## complementary-native-current.log

```text
error[E0599]: no associated function or constant named `__dsl_spec` found for struct `standards::v1::subsets::any::schema::diff::component::Fem2dDiff` in the current scope
  --> 🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/💾️binary/🔺️diff/🦀️.rs:6:1
   |
 6 | semio_framework_os_kernel::diff_binary!(crate::standards::v1::subsets::any::schema::diff::Fem2dDiff);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ associated function or constant not found in `standards::v1::subsets::any::schema::diff::component::Fem2dDiff`
   |
  ::: 🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🔺️diff/🦀️.rs:12:1
   |
12 | pub struct Fem2dDiff {
   | -------------------- associated function or constant `__dsl_spec` not found for this struct
   |
   = note: this error originates in the macro `semio_framework_os_kernel::diff_binary` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0599]: no method named `__dsl_to_record` found for reference `&standards::v1::subsets::any::schema::diff::component::Fem2dDiff` in the current scope
```

## complementary-native-current.log

```text
error[E0599]: no method named `__dsl_to_record` found for reference `&standards::v1::subsets::any::schema::diff::component::Fem2dDiff` in the current scope
 --> 🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/💾️binary/🔺️diff/🦀️.rs:6:1
  |
6 | semio_framework_os_kernel::diff_binary!(crate::standards::v1::subsets::any::schema::diff::Fem2dDiff);
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&standards::v1::subsets::any::schema::diff::component::Fem2dDiff`
  |
  = note: this error originates in the macro `semio_framework_os_kernel::diff_binary` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0599]: no associated function or constant named `__dsl_from_record` found for struct `standards::v1::subsets::any::schema::diff::component::Fem2dDiff` in the current scope
  --> 🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/💾️binary/🔺️diff/🦀️.rs:6:1
   |
 6 | semio_framework_os_kernel::diff_binary!(crate::standards::v1::subsets::any::schema::diff::Fem2dDiff);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ associated function or constant not found in `standards::v1::subsets::any::schema::diff::component::Fem2dDiff`
   |
```

## complementary-native-current.log

```text
error[E0599]: no associated function or constant named `__dsl_from_record` found for struct `standards::v1::subsets::any::schema::diff::component::Fem2dDiff` in the current scope
  --> 🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/💾️binary/🔺️diff/🦀️.rs:6:1
   |
 6 | semio_framework_os_kernel::diff_binary!(crate::standards::v1::subsets::any::schema::diff::Fem2dDiff);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ associated function or constant not found in `standards::v1::subsets::any::schema::diff::component::Fem2dDiff`
   |
  ::: 🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🔺️diff/🦀️.rs:12:1
   |
12 | pub struct Fem2dDiff {
   | -------------------- associated function or constant `__dsl_from_record` not found for this struct
   |
   = note: this error originates in the macro `semio_framework_os_kernel::diff_binary` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0004]: non-exhaustive patterns: `&dsl::semio_framework_io_sqlite_snapshot::artifact::Cell::PagedText(_)` not covered
```

## complementary-native-current.log

```text
error[E0004]: non-exhaustive patterns: `&dsl::semio_framework_io_sqlite_snapshot::artifact::Cell::PagedText(_)` not covered
  --> 🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🪶️sqlite/📸️snapshot/../../../../../../../../../🧩️sqlite/🧮️projection/🦀️.rs:19:35
   |
19 | ...{let size=match cell{Cell::Null=>0,Cell::Integer(_)|Cell::Real(_)|Cell::Float32(_)=>8,Cell::Text(value)=>value.len(),Cell::Blob(v...
   |                    ^^^^ pattern `&dsl::semio_framework_io_sqlite_snapshot::artifact::Cell::PagedText(_)` not covered
   |
note: `dsl::semio_framework_io_sqlite_snapshot::artifact::Cell<'_>` defined here
  --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/📦️packages/🦀️rust/../../🧩️artifact/🦀️.rs:46:1
   |
46 | pub enum Cell<'a> { Null, Integer(i64), Real(f64), Float32(f32), Text(&'a str), PagedText(&'a dyn semio_framework_value::paged::Utf8...
   | ^^^^^^^^^^^^^^^^^                                                               --------- not covered
   = note: the matched value is of type `&dsl::semio_framework_io_sqlite_snapshot::artifact::Cell<'_>`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
   |
```

## complementary-native-current.log

```text
error[E0609]: no field `finished` on type `os_store::component::ReplayStep`
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:13318:61
      |
13318 |     while !replay.step(&envelope.vcs.edits, &mut || false)?.finished {}
      |                                                             ^^^^^^^^ unknown field

warning: unused variable: `local_actor`
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:20635:75
      |
20635 |                     let ArtifactStoreBatchStage { edit, post, next_clock, local_actor, unit_flags, .. } = *stage;
      |                                                                           ^^^^^^^^^^^ help: try ignoring the field: `local_actor: _`
      |
      = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

```

## complementary-native-current.log

```text
error[E0425]: cannot find function `compute_energy_model_entries` in this scope
  --> 🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../.././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🗃️entries/🧪️tests/🔬️unit/🦀️.rs:18:19
   |
18 |     let entries = compute_energy_model_entries(&EnergyModelSnapshot::default());
   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function through its public re-export
   |
 1 + use crate::standards::v1::subsets::any::io::text::inferences::compute_energy_model_entries;
   |

error[E0425]: cannot find function `compute_energy_model_entries` in this scope
  --> 🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../.././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🗃️entries/🧪️tests/🔬️unit/🦀️.rs:27:19
   |
```

## complementary-native-current.log

```text
error[E0425]: cannot find function `compute_energy_model_entries` in this scope
  --> 🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../.././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🗃️entries/🧪️tests/🔬️unit/🦀️.rs:27:19
   |
27 |     let entries = compute_energy_model_entries(&snapshot);
   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function through its public re-export
   |
 1 + use crate::standards::v1::subsets::any::io::text::inferences::compute_energy_model_entries;
   |

error[E0425]: cannot find function `compute_energy_model_entries` in this scope
  --> 🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../.././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🗃️entries/🧪️tests/🔬️unit/🦀️.rs:38:19
   |
```

## complementary-native-current.log

```text
error[E0425]: cannot find function `compute_energy_model_entries` in this scope
  --> 🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../.././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🗃️entries/🧪️tests/🔬️unit/🦀️.rs:38:19
   |
38 |     let entries = compute_energy_model_entries(&snapshot);
   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function through its public re-export
   |
 1 + use crate::standards::v1::subsets::any::io::text::inferences::compute_energy_model_entries;
   |

error[E0425]: cannot find function `compute_energy_model_entries` in this scope
  --> 🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../.././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🗃️entries/🧪️tests/🔬️unit/🦀️.rs:39:25
   |
```

## complementary-native-current.log

```text
error[E0425]: cannot find function `compute_energy_model_entries` in this scope
  --> 🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../.././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🗃️entries/🧪️tests/🔬️unit/🦀️.rs:39:25
   |
39 |     assert_eq!(entries, compute_energy_model_entries(&snapshot));
   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function through its public re-export
   |
 1 + use crate::standards::v1::subsets::any::io::text::inferences::compute_energy_model_entries;
   |

error[E0425]: cannot find function `compute_energy_model_entries` in this scope
  --> 🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../.././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🗃️entries/🧪️tests/🔬️unit/🦀️.rs:46:16
   |
```

## complementary-native-current.log

```text
error[E0425]: cannot find function `compute_energy_model_entries` in this scope
  --> 🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../.././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🗃️entries/🧪️tests/🔬️unit/🦀️.rs:46:16
   |
46 |     assert_ne!(compute_energy_model_entries(&a).content_digest, compute_energy_model_entries(&b).content_digest);
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function through its public re-export
   |
 1 + use crate::standards::v1::subsets::any::io::text::inferences::compute_energy_model_entries;
   |

error[E0425]: cannot find function `compute_energy_model_entries` in this scope
  --> 🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../.././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🗃️entries/🧪️tests/🔬️unit/🦀️.rs:46:65
   |
```

## complementary-native-current.log

```text
error[E0425]: cannot find function `compute_energy_model_entries` in this scope
  --> 🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../.././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🗃️entries/🧪️tests/🔬️unit/🦀️.rs:46:65
   |
46 |     assert_ne!(compute_energy_model_entries(&a).content_digest, compute_energy_model_entries(&b).content_digest);
   |                                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function through its public re-export
   |
 1 + use crate::standards::v1::subsets::any::io::text::inferences::compute_energy_model_entries;
   |

warning: unnecessary qualification
   --> 🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/../../../../🔨️modules/⚡️simulation/⚙️engine/🌰️kernel/🧪️tests/🔬️unit/🦀️.rs:118:34
    |
```

## complementary-native-current.log

```text
error[E0753]: expected outer doc comment
 --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/🌍️geojson/🦀️.rs:2:1
  |
2 | //! 🌍️ gismap ← GeoJSON (RFC 7946, and GJ2008 files under stdio's CRS policy: CRS84/EPSG:4326 read as
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: inner doc comments like this (starting with `//!` or `/*!`) can only appear before items
help: you might have meant to write a regular comment
  |
2 - //! 🌍️ gismap ← GeoJSON (RFC 7946, and GJ2008 files under stdio's CRS policy: CRS84/EPSG:4326 read as
2 + // 🌍️ gismap ← GeoJSON (RFC 7946, and GJ2008 files under stdio's CRS policy: CRS84/EPSG:4326 read as
  |

error[E0753]: expected outer doc comment
```

## complementary-native-current.log

```text
error[E0753]: expected outer doc comment
 --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/🌍️geojson/🦀️.rs:3:1
  |
3 | //! lon/lat, spherical Web Mercator inverse projected, every other CRS refused) — `Point`s become
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: inner doc comments like this (starting with `//!` or `/*!`) can only appear before items
help: you might have meant to write a regular comment
  |
3 - //! lon/lat, spherical Web Mercator inverse projected, every other CRS refused) — `Point`s become
3 + // lon/lat, spherical Web Mercator inverse projected, every other CRS refused) — `Point`s become
  |

error[E0753]: expected outer doc comment
```

## complementary-native-current.log

```text
error[E0753]: expected outer doc comment
 --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/🌍️geojson/🦀️.rs:4:1
  |
4 | //! positions (`lon`, `lat`, and `alt` from a third coordinate), `LineString`s routes (`points`),
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: inner doc comments like this (starting with `//!` or `/*!`) can only appear before items
help: you might have meant to write a regular comment
  |
4 - //! positions (`lon`, `lat`, and `alt` from a third coordinate), `LineString`s routes (`points`),
4 + // positions (`lon`, `lat`, and `alt` from a third coordinate), `LineString`s routes (`points`),
  |

error[E0753]: expected outer doc comment
```

## complementary-native-current.log

```text
error[E0753]: expected outer doc comment
 --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/🌍️geojson/🦀️.rs:5:1
  |
5 | //! `Polygon`s regions (`ring` = the exterior without its closing position, `holes` = the interior rings
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: inner doc comments like this (starting with `//!` or `/*!`) can only appear before items
help: you might have meant to write a regular comment
  |
5 - //! `Polygon`s regions (`ring` = the exterior without its closing position, `holes` = the interior rings
5 + // `Polygon`s regions (`ring` = the exterior without its closing position, `holes` = the interior rings
  |

error[E0753]: expected outer doc comment
```

## complementary-native-current.log

```text
error[E0753]: expected outer doc comment
 --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/🌍️geojson/🦀️.rs:6:1
  |
6 | //! likewise). A multi-part geometry or GeometryCollection becomes one feature per part, id `<id>#<n>`.
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: inner doc comments like this (starting with `//!` or `/*!`) can only appear before items
help: you might have meant to write a regular comment
  |
6 - //! likewise). A multi-part geometry or GeometryCollection becomes one feature per part, id `<id>#<n>`.
6 + // likewise). A multi-part geometry or GeometryCollection becomes one feature per part, id `<id>#<n>`.
  |

error[E0753]: expected outer doc comment
```

## complementary-native-current.log

```text
error[E0753]: expected outer doc comment
 --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/🌍️geojson/🦀️.rs:7:1
  |
7 | //! Feature ids are kept (a number becomes its decimal text); a feature without one is `feature-<n>` by
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: inner doc comments like this (starting with `//!` or `/*!`) can only appear before items
help: you might have meant to write a regular comment
  |
7 - //! Feature ids are kept (a number becomes its decimal text); a feature without one is `feature-<n>` by
7 + // Feature ids are kept (a number becomes its decimal text); a feature without one is `feature-<n>` by
  |

error[E0753]: expected outer doc comment
```

## complementary-native-current.log

```text
error[E0753]: expected outer doc comment
 --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/🌍️geojson/🦀️.rs:8:1
  |
8 | //! its collection index; a repeated id within one family gains `#<n>`. `properties` become payload
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: inner doc comments like this (starting with `//!` or `/*!`) can only appear before items
help: you might have meant to write a regular comment
  |
8 - //! its collection index; a repeated id within one family gains `#<n>`. `properties` become payload
8 + // its collection index; a repeated id within one family gains `#<n>`. `properties` become payload
  |

error[E0753]: expected outer doc comment
```

## complementary-native-current.log

```text
error[E0753]: expected outer doc comment
 --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/🌍️geojson/🦀️.rs:9:1
  |
9 | //! members, and the geometry members win over properties of the same name.
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: inner doc comments like this (starting with `//!` or `/*!`) can only appear before items
help: you might have meant to write a regular comment
  |
9 - //! members, and the geometry members win over properties of the same name.
9 + // members, and the geometry members win over properties of the same name.
  |

error[E0753]: expected outer doc comment
```

## complementary-native-current.log

```text
error[E0753]: expected outer doc comment
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/🌍️geojson/🦀️.rs:10:1
   |
10 | //!
   | ^^^
   |
   = note: inner doc comments like this (starting with `//!` or `/*!`) can only appear before items
help: you might have meant to write a regular comment
   |
10 - //!
10 + //
   |

error[E0753]: expected outer doc comment
```

## complementary-native-current.log

```text
error[E0753]: expected outer doc comment
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/🌍️geojson/🦀️.rs:11:1
   |
11 | //! 🔖 `IoFidelity::Lossy`: features with `null` geometry carry no map feature and are dropped, as are
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: inner doc comments like this (starting with `//!` or `/*!`) can only appear before items
help: you might have meant to write a regular comment
   |
11 - //! 🔖 `IoFidelity::Lossy`: features with `null` geometry carry no map feature and are dropped, as are
11 + // 🔖 `IoFidelity::Lossy`: features with `null` geometry carry no map feature and are dropped, as are
   |

error[E0753]: expected outer doc comment
```

## complementary-native-current.log

```text
error[E0753]: expected outer doc comment
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/🌍️geojson/🦀️.rs:12:1
   |
12 | //! foreign members, `bbox`, the type of numeric ids, multi-part grouping, and property members named
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: inner doc comments like this (starting with `//!` or `/*!`) can only appear before items
help: you might have meant to write a regular comment
   |
12 - //! foreign members, `bbox`, the type of numeric ids, multi-part grouping, and property members named
12 + // foreign members, `bbox`, the type of numeric ids, multi-part grouping, and property members named
   |

error[E0753]: expected outer doc comment
```

## complementary-native-current.log

```text
error[E0753]: expected outer doc comment
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/🌍️geojson/🦀️.rs:13:1
   |
13 | //! like a geometry member.
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
14 | use crate::standards::v1::subsets::any::io::export::serializers::artifacts::json::v_rfc8259::geojson::GEOMETRY_MEMBERS;
   | ----------------------------------------------------------------------------------------------------------------------- the inner doc comment doesn't annotate this `use` import
   |
help: to annotate the `use` import, change the doc comment from inner to outer style
   |
13 - //! like a geometry member.
13 + /// like a geometry member.
   |

```

## complementary-native-current.log

```text
error[E0432]: unresolved import `crate::document_dsl`
 --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs:2:13
  |
2 | use crate::{document_dsl, MapFeature};
  |             ^^^^^^^^^^^^ no `document_dsl` in the root

error[E0432]: unresolved import `crate::schema::gis_map_descriptor_json`
 --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:2:21
  |
2 | use crate::schema::{gis_map_descriptor_json};
  |                     ^^^^^^^^^^^^^^^^^^^^^^^ no `gis_map_descriptor_json` in `schema`
  |
help: a similar name exists in the module
  |
```

## complementary-native-current.log

```text
error[E0432]: unresolved import `crate::schema::gis_map_descriptor_json`
 --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:2:21
  |
2 | use crate::schema::{gis_map_descriptor_json};
  |                     ^^^^^^^^^^^^^^^^^^^^^^^ no `gis_map_descriptor_json` in `schema`
  |
help: a similar name exists in the module
  |
2 - use crate::schema::{gis_map_descriptor_json};
2 + use crate::schema::{gis_map_descriptor_value};
  |

error[E0425]: cannot find type `Value` in this scope
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️relocated-engine/🦀️.rs:45:40
```

## complementary-native-current.log

```text
error[E0425]: cannot find type `Value` in this scope
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️relocated-engine/🦀️.rs:45:40
   |
45 |     let value = serde_json::from_str::<Value>(&semio_framework_pack_json::to_json_string(&document)).expect("document json");
   |                                        ^^^^^ not found in this scope
   |
note: struct `crate::standards::v1::subsets::any::io::binary::snapshot::owned_pack::Value` exists but is inaccessible
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/📦️pack/🦀️.rs:21:56
   |
21 | #[derive(semio_framework_dsl_record_derive::DslRecord)]struct Value{kind:Kind,boolean:Option<bool>,unsigned:Option<u64>,signed:Optio...
   |                                                        ^^^^^^^^^^^^ not accessible
help: consider importing one of these items
   |
 1 + use geojson::Value;
```

## complementary-native-current.log

```text
error[E0425]: cannot find type `Value` in this scope
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️relocated-engine/🦀️.rs:55:40
   |
55 |     let value = serde_json::from_str::<Value>(&semio_framework_pack_json::to_json_string(&GisMapSnapshot::default())).expect("empty ...
   |                                        ^^^^^ not found in this scope
   |
note: struct `crate::standards::v1::subsets::any::io::binary::snapshot::owned_pack::Value` exists but is inaccessible
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/📦️pack/🦀️.rs:21:56
   |
21 | #[derive(semio_framework_dsl_record_derive::DslRecord)]struct Value{kind:Kind,boolean:Option<bool>,unsigned:Option<u64>,signed:Optio...
   |                                                        ^^^^^^^^^^^^ not accessible
help: consider importing one of these items
   |
 1 + use geojson::Value;
```

## complementary-native-current.log

```text
error[E0425]: cannot find type `GIS_MAP_OWNED_FIELD_BYTES` in this scope
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:92:51
    |
 92 |               Decode(store::OwnedSchemaHexAuthority<GIS_MAP_OWNED_FIELD_BYTES>),
    |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
...
215 | / gis_map_owned_field_authority!(
216 | |     GisMapSnapshotDecodeState,
217 | |     GisMapSnapshotDecodeAuthority,
218 | |     GisMapSnapshot,
...   |
224 | |     "snapshot"
225 | | );
    | |_- in this macro invocation
```

## complementary-native-current.log

```text
error[E0425]: cannot find value `GIS_MAP_OWNED_FIELD_BYTES` in this scope
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:73:57
    |
 73 |               Ok(usize::from(self.retirement.is_some()) * GIS_MAP_OWNED_FIELD_BYTES)
    |                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
...
215 | / gis_map_owned_field_authority!(
216 | |     GisMapSnapshotDecodeState,
217 | |     GisMapSnapshotDecodeAuthority,
218 | |     GisMapSnapshot,
...   |
224 | |     "snapshot"
225 | | );
    | |_- in this macro invocation
```

## complementary-native-current.log

```text
error[E0425]: cannot find value `GisMapSnapshotRetirementFactory` in this scope
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:223:6
    |
223 |     &GisMapSnapshotRetirementFactory,
    |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
    |
help: consider importing this unit struct
    |
 15 + use crate::host::owned::GisMapSnapshotRetirementFactory;
    |

error[E0425]: cannot find type `GIS_MAP_OWNED_FIELD_BYTES` in this scope
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:92:51
    |
```

## complementary-native-current.log

```text
error[E0425]: cannot find value `GisMapMutationRetirementFactory` in this scope
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:235:6
    |
235 |     &GisMapMutationRetirementFactory,
    |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
    |
help: consider importing this unit struct
    |
 15 + use crate::host::owned::GisMapMutationRetirementFactory;
    |

error[E0425]: cannot find function `gis_map_document_store_owners` in this scope
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:49:47
   |
```

## complementary-native-current.log

```text
error[E0425]: cannot find function `gis_map_document_store_owners` in this scope
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:49:47
   |
49 |     store.install_document_store_owners_exact(gis_map_document_store_owners());
   |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this function
   |
 1 + use crate::host::owned::gis_map_document_store_owners;
   |

error[E0425]: cannot find type `GisMapStoreInitializationAuthority` in this scope
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:61:123
   |
```

## complementary-native-current.log

```text
error[E0425]: cannot find type `GisMapStoreInitializationAuthority` in this scope
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:61:123
   |
61 | ...emio_framework_job::Generation) -> GisMapStoreInitializationAuthority {
   |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
   |
note: struct `crate::host::owned::GisMapStoreInitializationAuthority` exists but is inaccessible
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././🔨️modules/🏠️host/🧰️owned/🦀️.rs:15:1
   |
15 | struct GisMapStoreInitializationAuthority {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not accessible

error[E0433]: cannot find type `GisMapStoreInitializationAuthority` in this scope
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:63:5
```

## complementary-native-current.log

```text
error[E0433]: cannot find type `GisMapStoreInitializationAuthority` in this scope
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:63:5
   |
63 |     GisMapStoreInitializationAuthority::new(envelope, operation, generation, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()))
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `GisMapStoreInitializationAuthority`
   |
note: struct `crate::host::owned::GisMapStoreInitializationAuthority` exists but is inaccessible
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././🔨️modules/🏠️host/🧰️owned/🦀️.rs:15:1
   |
15 | struct GisMapStoreInitializationAuthority {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not accessible

error[E0425]: cannot find type `GisMapStoreInitializationAuthority` in this scope
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:66:46
```

## complementary-native-current.log

```text
error[E0425]: cannot find type `GisMapStoreInitializationAuthority` in this scope
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:66:46
   |
66 | fn drive_gis_map_initializer(authority: &mut GisMapStoreInitializationAuthority, operation: semio_framework_job::OperationId, genera...
   |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
   |
note: struct `crate::host::owned::GisMapStoreInitializationAuthority` exists but is inaccessible
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././🔨️modules/🏠️host/🧰️owned/🦀️.rs:15:1
   |
15 | struct GisMapStoreInitializationAuthority {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not accessible

error[E0425]: cannot find value `GIS_MAP_OWNED_FIELD_BYTES` in this scope
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:84:54
```

## complementary-native-current.log

```text
error[E0425]: cannot find value `GIS_MAP_OWNED_FIELD_BYTES` in this scope
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:84:54
    |
 84 |         match disposer.close_step(&mut candidate, 1, GIS_MAP_OWNED_FIELD_BYTES).expect("GIS candidate close step") {
    |                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
    |
note: constant `crate::host::owned::GIS_MAP_OWNED_FIELD_BYTES` exists but is inaccessible
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././🔨️modules/🏠️host/🧰️owned/🦀️.rs:340:1
    |
340 | const GIS_MAP_OWNED_FIELD_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not accessible

error[E0425]: cannot find value `GIS_MAP_OWNED_FIELD_BYTES` in this scope
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:87:43
```

## complementary-native-current.log

```text
error[E0425]: cannot find value `GIS_MAP_OWNED_FIELD_BYTES` in this scope
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:87:43
    |
 87 |                 assert!(released_bytes <= GIS_MAP_OWNED_FIELD_BYTES);
    |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
    |
note: constant `crate::host::owned::GIS_MAP_OWNED_FIELD_BYTES` exists but is inaccessible
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././🔨️modules/🏠️host/🧰️owned/🦀️.rs:340:1
    |
340 | const GIS_MAP_OWNED_FIELD_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not accessible

error[E0425]: cannot find value `GIS_MAP_OWNED_FIELD_BYTES` in this scope
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:153:44
```

## complementary-native-current.log

```text
error[E0425]: cannot find value `GIS_MAP_OWNED_FIELD_BYTES` in this scope
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:153:44
    |
153 |             match retirement.close_step(1, GIS_MAP_OWNED_FIELD_BYTES).expect("one nested GIS owner retires") {
    |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
    |
note: constant `crate::host::owned::GIS_MAP_OWNED_FIELD_BYTES` exists but is inaccessible
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././🔨️modules/🏠️host/🧰️owned/🦀️.rs:340:1
    |
340 | const GIS_MAP_OWNED_FIELD_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not accessible

error[E0425]: cannot find value `GIS_MAP_OWNED_FIELD_BYTES` in this scope
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:156:47
```

## complementary-native-current.log

```text
error[E0425]: cannot find value `GIS_MAP_OWNED_FIELD_BYTES` in this scope
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:156:47
    |
156 |                     assert!(released_bytes <= GIS_MAP_OWNED_FIELD_BYTES);
    |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
    |
note: constant `crate::host::owned::GIS_MAP_OWNED_FIELD_BYTES` exists but is inaccessible
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././🔨️modules/🏠️host/🧰️owned/🦀️.rs:340:1
    |
340 | const GIS_MAP_OWNED_FIELD_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not accessible

error[E0425]: cannot find value `GisMapSnapshotRetirementFactory` in this scope
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:171:69
```

## complementary-native-current.log

```text
error[E0425]: cannot find value `GisMapSnapshotRetirementFactory` in this scope
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:171:69
    |
171 |     drain(store::ArtifactOwnedValueRetirementFactory::retire_owned(&GisMapSnapshotRetirementFactory, snapshot));
    |                                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
    |
help: consider importing this unit struct
    |
  1 + use crate::host::owned::GisMapSnapshotRetirementFactory;
    |

error[E0425]: cannot find value `GisMapMutationRetirementFactory` in this scope
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:177:69
    |
```

## complementary-native-current.log

```text
error[E0425]: cannot find value `GisMapMutationRetirementFactory` in this scope
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:177:69
    |
177 |     drain(store::ArtifactOwnedValueRetirementFactory::retire_owned(&GisMapMutationRetirementFactory, mutation));
    |                                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
    |
help: consider importing this unit struct
    |
  1 + use crate::host::owned::GisMapMutationRetirementFactory;
    |

error[E0425]: cannot find value `GisMapMutationRetirementFactory` in this scope
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:198:88
    |
```

## complementary-native-current.log

```text
error[E0425]: cannot find value `GisMapMutationRetirementFactory` in this scope
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:198:88
    |
198 |         let mut retirement = store::ArtifactOwnedValueRetirementFactory::retire_owned(&GisMapMutationRetirementFactory, mutation);
    |                                                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
    |
help: consider importing this unit struct
    |
  1 + use crate::host::owned::GisMapMutationRetirementFactory;
    |

error[E0425]: cannot find value `GIS_MAP_OWNED_FIELD_BYTES` in this scope
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:199:51
    |
```

## complementary-native-current.log

```text
error[E0425]: cannot find value `GIS_MAP_OWNED_FIELD_BYTES` in this scope
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:199:51
    |
199 | ...   assert!(matches!(retirement.close_step(0, GIS_MAP_OWNED_FIELD_BYTES).expect("zero-grant GIS retirement"), store::SnapshotReti...
    |                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
    |
note: constant `crate::host::owned::GIS_MAP_OWNED_FIELD_BYTES` exists but is inaccessible
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././🔨️modules/🏠️host/🧰️owned/🦀️.rs:340:1
    |
340 | const GIS_MAP_OWNED_FIELD_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not accessible

error[E0425]: cannot find value `GIS_MAP_OWNED_FIELD_BYTES` in this scope
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:201:44
```

## complementary-native-current.log

```text
error[E0425]: cannot find value `GIS_MAP_OWNED_FIELD_BYTES` in this scope
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:201:44
    |
201 |             match retirement.close_step(1, GIS_MAP_OWNED_FIELD_BYTES).expect("one catalog owner retires") {
    |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
    |
note: constant `crate::host::owned::GIS_MAP_OWNED_FIELD_BYTES` exists but is inaccessible
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././🔨️modules/🏠️host/🧰️owned/🦀️.rs:340:1
    |
340 | const GIS_MAP_OWNED_FIELD_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not accessible

error[E0425]: cannot find value `GIS_MAP_OWNED_FIELD_BYTES` in this scope
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:204:47
```

## complementary-native-current.log

```text
error[E0425]: cannot find value `GIS_MAP_OWNED_FIELD_BYTES` in this scope
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:204:47
    |
204 |                     assert!(released_bytes <= GIS_MAP_OWNED_FIELD_BYTES);
    |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
    |
note: constant `crate::host::owned::GIS_MAP_OWNED_FIELD_BYTES` exists but is inaccessible
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././🔨️modules/🏠️host/🧰️owned/🦀️.rs:340:1
    |
340 | const GIS_MAP_OWNED_FIELD_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not accessible

error[E0433]: cannot find type `GisMapStoreInitializationAuthority` in this scope
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:258:29
```

## complementary-native-current.log

```text
error[E0433]: cannot find type `GisMapStoreInitializationAuthority` in this scope
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:258:29
    |
258 | ...   let mut authority = GisMapStoreInitializationAuthority::new(envelope, operation, generation, protocol::ActorId(protocol::LOCA...
    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `GisMapStoreInitializationAuthority`
    |
note: struct `crate::host::owned::GisMapStoreInitializationAuthority` exists but is inaccessible
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././🔨️modules/🏠️host/🧰️owned/🦀️.rs:15:1
    |
 15 | struct GisMapStoreInitializationAuthority {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not accessible
help: a trait with a similar name exists
    |
258 -         let mut authority = GisMapStoreInitializationAuthority::new(envelope, operation, generation, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()));
```

## complementary-native-current.log

```text
error[E0433]: cannot find `owned_pack` in `snapshot`
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:42:301
   |
42 | ...::subsets::any::io::sqlite::snapshot::owned_pack::retire_value(old);
   |                                          ^^^^^^^^^^ could not find `owned_pack` in `snapshot`
   |
help: consider importing this module
   |
 1 + use crate::standards::v1::subsets::any::io::binary::snapshot::owned_pack;
   |
help: if you import `owned_pack`, refer to it directly
   |
42 -  let mut value=fixture();let old=std::mem::replace(&mut value.positions[0].data,semio_framework_value::DslValue::Array((0..600).map(|i|semio_framework_value::DslValue::String(if i==0{"long 世界".repeat(20000)}else{"child".into()})).collect()));crate::standards::v1::subsets::any::io::sqlite::snapshot::owned_pack::retire_value(old);
42 +  let mut value=fixture();let old=std::mem::replace(&mut value.positions[0].data,semio_framework_value::DslValue::Array((0..600).map(|i|semio_framework_value::DslValue::String(if i==0{"long 世界".repeat(20000)}else{"child".into()})).collect()));owned_pack::retire_value(old);
```

## complementary-native-current.log

```text
error[E0433]: cannot find `owned_pack` in `snapshot`
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:84:215
   |
84 | ...::subsets::any::io::sqlite::snapshot::owned_pack::retire_value(old);
   |                                          ^^^^^^^^^^ could not find `owned_pack` in `snapshot`
   |
help: consider importing this module
   |
 1 + use crate::standards::v1::subsets::any::io::binary::snapshot::owned_pack;
   |
help: if you import `owned_pack`, refer to it directly
   |
84 -  let mut snapshot=fixture();let old=std::mem::replace(&mut snapshot.positions[0].data,semio_framework_value::DslValue::String("interior 世界".repeat(20000)));crate::standards::v1::subsets::any::io::sqlite::snapshot::owned_pack::retire_value(old);
84 +  let mut snapshot=fixture();let old=std::mem::replace(&mut snapshot.positions[0].data,semio_framework_value::DslValue::String("interior 世界".repeat(20000)));owned_pack::retire_value(old);
```

## complementary-native-current.log

```text
error[E0433]: cannot find type `GisMapSnapshotDecodeAuthority` in this scope
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././🔨️modules/🏠️host/🧰️owned/🦀️.rs:968:18
    |
968 |         Box::new(GisMapSnapshotDecodeAuthority::new(operation, generation, path))
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `GisMapSnapshotDecodeAuthority`
    |
note: struct `crate::standards::v1::subsets::any::io::binary::mutations::GisMapSnapshotDecodeAuthority` exists but is inaccessible
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:99:9
    |
 99 |           struct $authority {
    |           ^^^^^^^^^^^^^^^^^ not accessible
...
215 | / gis_map_owned_field_authority!(
216 | |     GisMapSnapshotDecodeState,
```

## complementary-native-current.log

```text
error[E0433]: cannot find type `GisMapMutationDecodeAuthority` in this scope
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././🔨️modules/🏠️host/🧰️owned/🦀️.rs:972:18
    |
972 |         Box::new(GisMapMutationDecodeAuthority::new(operation, generation, path))
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `GisMapMutationDecodeAuthority`
    |
note: struct `crate::standards::v1::subsets::any::io::binary::mutations::GisMapMutationDecodeAuthority` exists but is inaccessible
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:99:9
    |
 99 |           struct $authority {
    |           ^^^^^^^^^^^^^^^^^ not accessible
...
227 | / gis_map_owned_field_authority!(
228 | |     GisMapMutationDecodeState,
```

## complementary-native-current.log

```text
error[E0747]: unresolved item provided when a constant was expected
   --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:92:51
    |
 92 |               Decode(store::OwnedSchemaHexAuthority<GIS_MAP_OWNED_FIELD_BYTES>),
    |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^
...
215 | / gis_map_owned_field_authority!(
216 | |     GisMapSnapshotDecodeState,
217 | |     GisMapSnapshotDecodeAuthority,
218 | |     GisMapSnapshot,
...   |
224 | |     "snapshot"
225 | | );
    | |_- in this macro invocation
```

## complementary-native-current.log

```text
error[E0282]: type annotations needed
  --> 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/🌍️geojson/🦀️.rs:42:9
   |
42 | ...et properties = payload.iter().filter(|(key, _)| !GEOMETRY_MEMBERS.contains(&key.as_str())).map(|(key, value)| (key.clone(), value....
   |       ^^^^^^^^^^
43 | ...k(GeoJsonFeature { id: Some(GeoJsonId::Text(map_feature.id.clone())), geometry: geometry(&payload)?, properties: Some(properties.in...
   |                                                                                                                          ---------- type must be known at this point
   |
help: consider giving `properties` an explicit type
   |
42 |     let properties: Vec<_> = payload.iter().filter(|(key, _)| !GEOMETRY_MEMBERS.contains(&key.as_str())).map(|(key, value)| (key.clone(), value.clone())).collect();
   |                   ++++++++

[cargo:build] running elapsedMs=1270245
```

## complementary-native-current.log

```text
error[E0432]: unresolved import `mutations_wire_codec`
  --> 🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:47:9
   |
47 | pub use mutations_wire_codec::*;
   |         ^^^^^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `mutations_wire_codec`
   |
   = help: if you wanted to use a crate named `mutations_wire_codec`, use `cargo add mutations_wire_codec` to add it to your `Cargo.toml`

error[E0603]: struct import `FormsSnapshot` is private
   --> 🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust/../.././🔨️modules/🏠️host/🧰️owned/🦀️.rs:5:64
    |
  5 | use crate::standards::v1::subsets::any::io::binary::snapshot::{FormsSnapshot};
    |                                                                ^^^^^^^^^^^^^ private struct import
    |
```

## complementary-native-current.log

```text
error[E0603]: struct import `FormsSnapshot` is private
   --> 🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust/../.././🔨️modules/🏠️host/🧰️owned/🦀️.rs:5:64
    |
  5 | use crate::standards::v1::subsets::any::io::binary::snapshot::{FormsSnapshot};
    |                                                                ^^^^^^^^^^^^^ private struct import
    |
note: the struct import `FormsSnapshot` is defined here...
   --> 🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🦀️.rs:12:5
    |
 12 | use crate::FormsSnapshot;
    |     ^^^^^^^^^^^^^^^^^^^^
note: ...and refers to the struct import `FormsSnapshot` which is defined here...
   --> 🔌️plugins/📋️forms/🗿️artifacts/📋️forms/📦️packages/🦀️rust/../../🦀️.rs:35:9
    |
```

## complementary-native-current.log

```text
error[E0432]: unresolved import `semio_s_artifact_stdio_gltf::standards::v2_0::subsets::any::io::GltfAccessorType`
 --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../.././././././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️.rs:7:99
  |
7 | use semio_s_artifact_stdio_gltf::standards::v2_0::subsets::any::io::{decode_accessor, decode_glb, GltfAccessorType};
  |                                                                                                   ^^^^^^^^^^^^^^^^ no `GltfAccessorType` in `standards::v2_0::subsets::any::io`
  |
  = help: consider importing this enum instead:
          semio_s_artifact_stdio_gltf::engine::snapshot::GltfAccessorType
help: a similar name exists in the module
  |
7 - use semio_s_artifact_stdio_gltf::standards::v2_0::subsets::any::io::{decode_accessor, decode_glb, GltfAccessorType};
7 + use semio_s_artifact_stdio_gltf::standards::v2_0::subsets::any::io::{decode_accessor, decode_glb, GltfAccessorSpec};
  |

```

## complementary-native-current.log

```text
error[E0432]: unresolved imports `semio_s_artifact_stdio_gltf::standards::v2_0::subsets::any::io::GltfAccessorType`, `semio_s_artifact_stdio_gltf::standards::v2_0::subsets::any::io::GltfComponentType`
 --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../.././././././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️.rs:9:82
  |
9 | use semio_s_artifact_stdio_gltf::standards::v2_0::subsets::any::io::{encode_glb, GltfAccessorType, GltfComponentType};
  |                                                                                  ^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^ no `GltfComponentType` in `standards::v2_0::subsets::any::io`
  |                                                                                  |
  |                                                                                  no `GltfAccessorType` in `standards::v2_0::subsets::any::io`
  |
  = help: consider importing this enum instead:
          semio_s_artifact_stdio_gltf::engine::snapshot::GltfAccessorType
  = help: consider importing this enum instead:
          semio_s_artifact_stdio_gltf::engine::snapshot::GltfComponentType
help: a similar name exists in the module
  |
```

## complementary-native-current.log

```text
error[E0433]: cannot find `pack` in `snapshot`
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../🦀️.rs:427:46
    |
427 |                     protocol: Some(snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
    |                                              ^^^^ could not find `pack` in `snapshot`

error[E0433]: cannot find `pack` in `snapshot`
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../🦀️.rs:428:51
    |
428 |                     protocol_path: Some(snapshot::pack::COMPONENT_PROTOCOL_PATH),
    |                                                   ^^^^ could not find `pack` in `snapshot`

error[E0433]: cannot find `pack` in `snapshot`
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../🦀️.rs:457:46
```

## complementary-native-current.log

```text
error[E0433]: cannot find `pack` in `snapshot`
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../🦀️.rs:428:51
    |
428 |                     protocol_path: Some(snapshot::pack::COMPONENT_PROTOCOL_PATH),
    |                                                   ^^^^ could not find `pack` in `snapshot`

error[E0433]: cannot find `pack` in `snapshot`
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../🦀️.rs:457:46
    |
457 |                     protocol: Some(snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
    |                                              ^^^^ could not find `pack` in `snapshot`

error[E0433]: cannot find `pack` in `snapshot`
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../🦀️.rs:458:51
```

## complementary-native-current.log

```text
error[E0433]: cannot find `pack` in `snapshot`
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../🦀️.rs:457:46
    |
457 |                     protocol: Some(snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
    |                                              ^^^^ could not find `pack` in `snapshot`

error[E0433]: cannot find `pack` in `snapshot`
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../🦀️.rs:458:51
    |
458 |                     protocol_path: Some(snapshot::pack::COMPONENT_PROTOCOL_PATH),
    |                                                   ^^^^ could not find `pack` in `snapshot`

error[E0425]: cannot find value `COMPONENT_GRAMMAR_SEMIO` in module `diff`
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../🦀️.rs:445:41
```

## complementary-native-current.log

```text
error[E0433]: cannot find `pack` in `snapshot`
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../🦀️.rs:458:51
    |
458 |                     protocol_path: Some(snapshot::pack::COMPONENT_PROTOCOL_PATH),
    |                                                   ^^^^ could not find `pack` in `snapshot`

error[E0425]: cannot find value `COMPONENT_GRAMMAR_SEMIO` in module `diff`
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../🦀️.rs:445:41
    |
445 |                     grammar: Some(diff::COMPONENT_GRAMMAR_SEMIO),
    |                                         ^^^^^^^^^^^^^^^^^^^^^^^ not found in `diff`
    |
help: consider importing one of these constants
    |
```

## complementary-native-current.log

```text
error[E0425]: cannot find value `COMPONENT_GRAMMAR_SEMIO` in module `diff`
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../🦀️.rs:445:41
    |
445 |                     grammar: Some(diff::COMPONENT_GRAMMAR_SEMIO),
    |                                         ^^^^^^^^^^^^^^^^^^^^^^^ not found in `diff`
    |
help: consider importing one of these constants
    |
 14 + use crate::standards::v1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_SEMIO;
    |
 14 + use crate::standards::v1::subsets::any::io::text::inferences::COMPONENT_GRAMMAR_SEMIO;
    |
 14 + use crate::standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO;
    |
```

## complementary-native-current.log

```text
error[E0425]: cannot find value `COMPONENT_GRAMMAR_PATH` in module `diff`
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../🦀️.rs:446:46
    |
446 |                     grammar_path: Some(diff::COMPONENT_GRAMMAR_PATH),
    |                                              ^^^^^^^^^^^^^^^^^^^^^^ not found in `diff`
    |
help: consider importing one of these constants
    |
 14 + use crate::standards::v1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_PATH;
    |
 14 + use crate::standards::v1::subsets::any::io::text::inferences::COMPONENT_GRAMMAR_PATH;
    |
 14 + use crate::standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_PATH;
    |
```

## complementary-native-current.log

```text
error[E0277]: the trait bound `semio_framework_plugin::MeshAttributeDomain: BorrowedDslField` is not satisfied
  --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🕸️mesh/🦀️.rs:27:16
   |
27 |     pub domain:MeshAttributeDomain,
   |                ^^^^^^^^^^^^^^^^^^^ the trait `BorrowedDslField` is not implemented for `semio_framework_plugin::MeshAttributeDomain`
   |
   = help: the following other types implement trait `BorrowedDslField`:
             AddPaintLayer
             ArtifactChild<S>
             ArtifactLink
             BTreeMap<std::string::String, T>
             CanvasPointerDown
             CanvasPointerMove
             CanvasPointerUp
```

## complementary-native-current.log

```text
error[E0277]: the trait bound `semio_framework_plugin::MeshAttributeSemantic: BorrowedDslField` is not satisfied
  --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🕸️mesh/🦀️.rs:28:18
   |
28 |     pub semantic:MeshAttributeSemantic,
   |                  ^^^^^^^^^^^^^^^^^^^^^ the trait `BorrowedDslField` is not implemented for `semio_framework_plugin::MeshAttributeSemantic`
   |
   = help: the following other types implement trait `BorrowedDslField`:
             AddPaintLayer
             ArtifactChild<S>
             ArtifactLink
             BTreeMap<std::string::String, T>
             CanvasPointerDown
             CanvasPointerMove
             CanvasPointerUp
```

## complementary-native-current.log

```text
error[E0277]: the trait bound `semio_framework_plugin::MeshAttributeInterpolation: BorrowedDslField` is not satisfied
  --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🕸️mesh/🦀️.rs:29:23
   |
29 |     pub interpolation:MeshAttributeInterpolation,
   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `BorrowedDslField` is not implemented for `semio_framework_plugin::MeshAttributeInterpolation`
   |
   = help: the following other types implement trait `BorrowedDslField`:
             AddPaintLayer
             ArtifactChild<S>
             ArtifactLink
             BTreeMap<std::string::String, T>
             CanvasPointerDown
             CanvasPointerMove
             CanvasPointerUp
```

## complementary-native-current.log

```text
error[E0063]: missing field `snapshot_owner` in initializer of `ArtifactCommandInputs<'_, _>`
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:257:20
    |
257 |             .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs {
    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `snapshot_owner`

error[E0063]: missing field `snapshot_owner` in initializer of `ArtifactCommandInputs<'_, _>`
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:278:20
    |
278 |             .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs {
    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `snapshot_owner`

error[E0063]: missing field `snapshot_owner` in initializer of `ArtifactCommandInputs<'_, _>`
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:293:20
```

## complementary-native-current.log

```text
error[E0063]: missing field `snapshot_owner` in initializer of `ArtifactCommandInputs<'_, _>`
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:278:20
    |
278 |             .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs {
    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `snapshot_owner`

error[E0063]: missing field `snapshot_owner` in initializer of `ArtifactCommandInputs<'_, _>`
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:293:20
    |
293 |             .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs {
    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `snapshot_owner`

error[E0063]: missing field `snapshot_owner` in initializer of `ArtifactCommandInputs<'_, _>`
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:309:16
```

## complementary-native-current.log

```text
error[E0063]: missing field `snapshot_owner` in initializer of `ArtifactCommandInputs<'_, _>`
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:293:20
    |
293 |             .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs {
    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `snapshot_owner`

error[E0063]: missing field `snapshot_owner` in initializer of `ArtifactCommandInputs<'_, _>`
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:309:16
    |
309 | ...   .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs { command: &command, snapshot: &snapshot, config: &con...
    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `snapshot_owner`

error[E0063]: missing field `snapshot_owner` in initializer of `ArtifactCommandInputs<'_, _>`
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:313:16
```

## complementary-native-current.log

```text
error[E0063]: missing field `snapshot_owner` in initializer of `ArtifactCommandInputs<'_, _>`
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:309:16
    |
309 | ...   .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs { command: &command, snapshot: &snapshot, config: &con...
    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `snapshot_owner`

error[E0063]: missing field `snapshot_owner` in initializer of `ArtifactCommandInputs<'_, _>`
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:313:16
    |
313 |         .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs {
    |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `snapshot_owner`

error[E0063]: missing field `snapshot_owner` in initializer of `ArtifactCommandInputs<'_, _>`
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:326:20
```

## complementary-native-current.log

```text
error[E0063]: missing field `snapshot_owner` in initializer of `ArtifactCommandInputs<'_, _>`
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:313:16
    |
313 |         .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs {
    |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `snapshot_owner`

error[E0063]: missing field `snapshot_owner` in initializer of `ArtifactCommandInputs<'_, _>`
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:326:20
    |
326 |             .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs {
    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `snapshot_owner`

error[E0599]: no method named `to_uri` found for struct `semio_framework_artifact_reference::ArtifactRef` in the current scope
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:717:126
```

## complementary-native-current.log

```text
error[E0063]: missing field `snapshot_owner` in initializer of `ArtifactCommandInputs<'_, _>`
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:326:20
    |
326 |             .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs {
    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `snapshot_owner`

error[E0599]: no method named `to_uri` found for struct `semio_framework_artifact_reference::ArtifactRef` in the current scope
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:717:126
    |
717 | ...IELD_BYTES && mesh.target.to_uri().len() <= LOWPOLY_RETAINED_FIELD_BYTES)
    |                              ^^^^^^ method not found in `semio_framework_artifact_reference::ArtifactRef`
    |
   ::: /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust/././../../../🚪️io/📝️text/🗿️artifact-reference/🦀️.rs:11:5
    |
```

## complementary-native-current.log

```text
error[E0599]: no method named `to_uri` found for struct `semio_framework_artifact_reference::ArtifactRef` in the current scope
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:717:126
    |
717 | ...IELD_BYTES && mesh.target.to_uri().len() <= LOWPOLY_RETAINED_FIELD_BYTES)
    |                              ^^^^^^ method not found in `semio_framework_artifact_reference::ArtifactRef`
    |
   ::: /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust/././../../../🚪️io/📝️text/🗿️artifact-reference/🦀️.rs:11:5
    |
 11 |  fn to_uri(&self)->String;
    |     ------ the method is available for `semio_framework_artifact_reference::ArtifactRef` here
    |
    = help: items from traits can only be used if the trait is in scope
help: trait `ArtifactReferenceText` which provides `to_uri` is implemented but not in scope; perhaps you want to import it
    |
```

## complementary-native-current.log

```text
error[E0599]: no method named `to_uri` found for struct `semio_framework_artifact_reference::ArtifactRef` in the current scope
    --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1193:110
     |
1193 |         .saturating_add(object.mesh.as_ref().map_or(0, |mesh| mesh.child_id.len().saturating_add(mesh.target.to_uri().len())))
     |                                                                                                              ^^^^^^ method not found in `semio_framework_artifact_reference::ArtifactRef`
     |
    ::: /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust/././../../../🚪️io/📝️text/🗿️artifact-reference/🦀️.rs:11:5
     |
  11 |  fn to_uri(&self)->String;
     |     ------ the method is available for `semio_framework_artifact_reference::ArtifactRef` here
     |
     = help: items from traits can only be used if the trait is in scope
help: trait `ArtifactReferenceText` which provides `to_uri` is implemented but not in scope; perhaps you want to import it
     |
```

## complementary-native-current.log

```text
error[E0599]: no method named `to_uri` found for struct `semio_framework_artifact_reference::ArtifactRef` in the current scope
    --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1218:138
     |
1218 | ...ating_add(payload.target.to_uri().len()).saturating_add(payload.mesh_workspace.len())),
     |                             ^^^^^^ method not found in `semio_framework_artifact_reference::ArtifactRef`
     |
    ::: /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust/././../../../🚪️io/📝️text/🗿️artifact-reference/🦀️.rs:11:5
     |
  11 |  fn to_uri(&self)->String;
     |     ------ the method is available for `semio_framework_artifact_reference::ArtifactRef` here
     |
     = help: items from traits can only be used if the trait is in scope
help: trait `ArtifactReferenceText` which provides `to_uri` is implemented but not in scope; perhaps you want to import it
     |
```

## complementary-native-current.log

```text
error[E0308]: mismatched types
  --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:34:134
   |
34 | ...med { what: "lowpoly-mutation", offset: error.valid_up_to(), detail: error.to_string() })?;
   |                                            ^^^^^^^^^^^^^^^^^^^ expected `u64`, found `usize`

error[E0433]: cannot find module or crate `document_dsl` in this scope
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../🦀️.rs:425:35
    |
425 |                     grammar: Some(document_dsl::COMPONENT_GRAMMAR_SEMIO),
    |                                   ^^^^^^^^^^^^ use of unresolved module or unlinked crate `document_dsl`
    |
    = help: if you wanted to use a crate named `document_dsl`, use `cargo add document_dsl` to add it to your `Cargo.toml`

```

## complementary-native-current.log

```text
error[E0433]: cannot find module or crate `document_dsl` in this scope
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../🦀️.rs:425:35
    |
425 |                     grammar: Some(document_dsl::COMPONENT_GRAMMAR_SEMIO),
    |                                   ^^^^^^^^^^^^ use of unresolved module or unlinked crate `document_dsl`
    |
    = help: if you wanted to use a crate named `document_dsl`, use `cargo add document_dsl` to add it to your `Cargo.toml`

error[E0433]: cannot find module or crate `document_dsl` in this scope
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../🦀️.rs:426:40
    |
426 |                     grammar_path: Some(document_dsl::COMPONENT_GRAMMAR_PATH),
    |                                        ^^^^^^^^^^^^ use of unresolved module or unlinked crate `document_dsl`
    |
```

## complementary-native-current.log

```text
error[E0433]: cannot find module or crate `document_dsl` in this scope
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../🦀️.rs:426:40
    |
426 |                     grammar_path: Some(document_dsl::COMPONENT_GRAMMAR_PATH),
    |                                        ^^^^^^^^^^^^ use of unresolved module or unlinked crate `document_dsl`
    |
    = help: if you wanted to use a crate named `document_dsl`, use `cargo add document_dsl` to add it to your `Cargo.toml`

error[E0433]: cannot find module or crate `op` in this scope
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../🦀️.rs:435:35
    |
435 |                     grammar: Some(op::COMPONENT_GRAMMAR_SEMIO),
    |                                   ^^ use of unresolved module or unlinked crate `op`
    |
```

## complementary-native-current.log

```text
error[E0433]: cannot find module or crate `op` in this scope
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../🦀️.rs:435:35
    |
435 |                     grammar: Some(op::COMPONENT_GRAMMAR_SEMIO),
    |                                   ^^ use of unresolved module or unlinked crate `op`
    |
    = help: if you wanted to use a crate named `op`, use `cargo add op` to add it to your `Cargo.toml`

error[E0433]: cannot find module or crate `op` in this scope
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../🦀️.rs:436:40
    |
436 |                     grammar_path: Some(op::COMPONENT_GRAMMAR_PATH),
    |                                        ^^ use of unresolved module or unlinked crate `op`
    |
```

## complementary-native-current.log

```text
error[E0433]: cannot find module or crate `op` in this scope
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../🦀️.rs:436:40
    |
436 |                     grammar_path: Some(op::COMPONENT_GRAMMAR_PATH),
    |                                        ^^ use of unresolved module or unlinked crate `op`
    |
    = help: if you wanted to use a crate named `op`, use `cargo add op` to add it to your `Cargo.toml`

error[E0433]: cannot find module or crate `spr` in this scope
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../🦀️.rs:437:36
    |
437 |                     protocol: Some(spr::COMPONENT_PROTOCOL_SEMIO),
    |                                    ^^^ use of unresolved module or unlinked crate `spr`
    |
```

## complementary-native-current.log

```text
error[E0433]: cannot find module or crate `spr` in this scope
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../🦀️.rs:437:36
    |
437 |                     protocol: Some(spr::COMPONENT_PROTOCOL_SEMIO),
    |                                    ^^^ use of unresolved module or unlinked crate `spr`
    |
    = help: if you wanted to use a crate named `spr`, use `cargo add spr` to add it to your `Cargo.toml`
help: a builtin type with a similar name exists
    |
437 -                     protocol: Some(spr::COMPONENT_PROTOCOL_SEMIO),
437 +                     protocol: Some(str::COMPONENT_PROTOCOL_SEMIO),
    |

error[E0433]: cannot find module or crate `spr` in this scope
```

## complementary-native-current.log

```text
error[E0433]: cannot find module or crate `spr` in this scope
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../🦀️.rs:438:41
    |
438 |                     protocol_path: Some(spr::COMPONENT_PROTOCOL_PATH),
    |                                         ^^^ use of unresolved module or unlinked crate `spr`
    |
    = help: if you wanted to use a crate named `spr`, use `cargo add spr` to add it to your `Cargo.toml`
help: a builtin type with a similar name exists
    |
438 -                     protocol_path: Some(spr::COMPONENT_PROTOCOL_PATH),
438 +                     protocol_path: Some(str::COMPONENT_PROTOCOL_PATH),
    |

error[E0433]: cannot find module or crate `spr` in this scope
```

## complementary-native-current.log

```text
error[E0433]: cannot find module or crate `spr` in this scope
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../🦀️.rs:467:36
    |
467 |                     protocol: Some(spr::COMPONENT_PROTOCOL_SEMIO),
    |                                    ^^^ use of unresolved module or unlinked crate `spr`
    |
    = help: if you wanted to use a crate named `spr`, use `cargo add spr` to add it to your `Cargo.toml`
help: a builtin type with a similar name exists
    |
467 -                     protocol: Some(spr::COMPONENT_PROTOCOL_SEMIO),
467 +                     protocol: Some(str::COMPONENT_PROTOCOL_SEMIO),
    |

error[E0433]: cannot find module or crate `spr` in this scope
```

## complementary-native-current.log

```text
error[E0433]: cannot find module or crate `spr` in this scope
   --> 🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/../../🦀️.rs:468:41
    |
468 |                     protocol_path: Some(spr::COMPONENT_PROTOCOL_PATH),
    |                                         ^^^ use of unresolved module or unlinked crate `spr`
    |
    = help: if you wanted to use a crate named `spr`, use `cargo add spr` to add it to your `Cargo.toml`
help: a builtin type with a similar name exists
    |
468 -                     protocol_path: Some(spr::COMPONENT_PROTOCOL_PATH),
468 +                     protocol_path: Some(str::COMPONENT_PROTOCOL_PATH),
    |

warning: unused import: `protocol::Mutation`
```

## complementary-native-current.log

```text
error[E0432]: unresolved import `semio_s_artifact_stdio_png::standards::v1_2::subsets::any::schema::snapshot::PngChunkMarker`
   --> 🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🦀️.rs:128:132
    |
128 | ... schema::snapshot::{PngChunkMarker, PngColorType}};
    |                        ^^^^^^^^^^^^^^ no `PngChunkMarker` in `standards::v1_2::subsets::any::schema::snapshot`
    |
    = help: consider importing this enum instead:
            semio_s_artifact_stdio_png::standards::v1_2::subsets::any::io::PngChunkMarker

error[E0432]: unresolved imports `crate::standards::v1::subsets::any::io::diff`, `crate::standards::v1::subsets::any::io::mutations`, `crate::standards::v1::subsets::any::io::snapshot`
  --> 🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs:23:50
   |
23 |     use crate::standards::v1::subsets::any::io::{diff, mutations, snapshot};
   |                                                  ^^^^  ^^^^^^^^^  ^^^^^^^^ no `snapshot` in `standards::v1::subsets::any::io`
```

## complementary-native-current.log

```text
error[E0432]: unresolved imports `crate::standards::v1::subsets::any::io::diff`, `crate::standards::v1::subsets::any::io::mutations`, `crate::standards::v1::subsets::any::io::snapshot`
  --> 🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs:23:50
   |
23 |     use crate::standards::v1::subsets::any::io::{diff, mutations, snapshot};
   |                                                  ^^^^  ^^^^^^^^^  ^^^^^^^^ no `snapshot` in `standards::v1::subsets::any::io`
   |                                                  |     |
   |                                                  |     no `mutations` in `standards::v1::subsets::any::io`
   |                                                  no `diff` in `standards::v1::subsets::any::io`
   |
   = help: consider importing one of these modules instead:
           crate::diff
           crate::mutations::change_seed::diff
           crate::mutations::change_tile_media::diff
           crate::mutations::change_tile_weight::diff
```

## complementary-native-current.log

```text
error[E0432]: unresolved imports `crate::standards::v1::subsets::any::io::mutations`, `crate::standards::v1::subsets::any::io::snapshot`
 --> 🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️unit/🦀️.rs:5:46
  |
5 | use crate::standards::v1::subsets::any::io::{mutations, snapshot};
  |                                              ^^^^^^^^^  ^^^^^^^^ no `snapshot` in `standards::v1::subsets::any::io`
  |                                              |
  |                                              no `mutations` in `standards::v1::subsets::any::io`
  |
  = help: consider importing one of these modules instead:
          crate::mutations
          crate::schema::mutations
          crate::standards::v1::subsets::any::io::binary::mutations
          crate::standards::v1::subsets::any::io::text::mutations
          semio_s_artifact_stdio_png::schema::mutations
```

## complementary-native-current.log

```text
error[E0432]: unresolved import `controlled_native`
   --> 🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🦀️.rs:143:15
    |
143 | pub(crate)use controlled_native::{decode_sqlite_snapshot_native,encode_sqlite_snapshot_native};
    |               ^^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `controlled_native`
    |
    = help: if you wanted to use a crate named `controlled_native`, use `cargo add controlled_native` to add it to your `Cargo.toml`

warning: unnecessary qualification
  --> 🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs:38:35
   |
38 |                     grammar: Some(crate::standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
```

## complementary-native-current.log

```text
error[E0432]: unresolved imports `crate::standards::v1::subsets::any::schema::mutations::binary`, `crate::standards::v1::subsets::any::schema::mutations::text`
 --> 🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️unit/🦀️.rs:7:61
  |
7 | use crate::standards::v1::subsets::any::schema::mutations::{binary as mutation_binary,text as mutation_text};
  |                                                             ------^^^^^^^^^^^^^^^^^^^ ----^^^^^^^^^^^^^^^^^
  |                                                             |                         |
  |                                                             |                         no `text` in `standards::v1::subsets::any::schema::mutations`
  |                                                             no `binary` in `standards::v1::subsets::any::schema::mutations`
  |
  = help: consider importing one of these modules instead:
          crate::dsl::os_spr::io::binary
          crate::standards::v1::subsets::any::io::binary
          semio_framework_os_kernel::os_spr::io::binary
          semio_s_artifact_stdio_semio::standards::v1::subsets::animation::io::binary
```

## complementary-native-current.log

```text
error[E0432]: unresolved imports `crate::standards::v1::subsets::any::schema::snapshot::binary`, `crate::standards::v1::subsets::any::schema::snapshot::text`
 --> 🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️unit/🦀️.rs:9:60
  |
9 | use crate::standards::v1::subsets::any::schema::snapshot::{binary as snapshot_binary, text as snapshot_text};
  |                                                            ------^^^^^^^^^^^^^^^^^^^  ----^^^^^^^^^^^^^^^^^
  |                                                            |                          |
  |                                                            |                          no `text` in `standards::v1::subsets::any::schema::snapshot`
  |                                                            no `binary` in `standards::v1::subsets::any::schema::snapshot`
  |
  = help: consider importing one of these modules instead:
          crate::dsl::os_spr::io::binary
          crate::standards::v1::subsets::any::io::binary
          semio_framework_os_kernel::os_spr::io::binary
          semio_s_artifact_stdio_semio::standards::v1::subsets::animation::io::binary
```

## complementary-native-current.log

```text
error[E0432]: unresolved import `controlled_native`
   --> 🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🦀️.rs:114:15
    |
114 | pub(crate)use controlled_native::{decode_sqlite_snapshot_native,encode_sqlite_snapshot_native};
    |               ^^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `controlled_native`
    |
    = help: if you wanted to use a crate named `controlled_native`, use `cargo add controlled_native` to add it to your `Cargo.toml`

error[E0433]: cannot find type `Wfc3dBuilderConstruction` in this scope
  --> 🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️unit/🦀️.rs:47:16
   |
47 |     assert_eq!(Wfc3dBuilderConstruction::from_text(&text).expect("text builds").build().expect("no diagnostics"), document);
   |                ^^^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `Wfc3dBuilderConstruction`
   |
```

## complementary-native-current.log

```text
error[E0433]: cannot find type `Wfc3dBuilderConstruction` in this scope
  --> 🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️unit/🦀️.rs:47:16
   |
47 |     assert_eq!(Wfc3dBuilderConstruction::from_text(&text).expect("text builds").build().expect("no diagnostics"), document);
   |                ^^^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `Wfc3dBuilderConstruction`
   |
help: consider importing this struct through its public re-export
   |
 4 + use crate::Wfc3dBuilderConstruction;
   |

error[E0433]: cannot find type `Wfc3dBuilderConstruction` in this scope
  --> 🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️unit/🦀️.rs:49:16
   |
```

## complementary-native-current.log

```text
error[E0433]: cannot find type `Wfc3dBuilderConstruction` in this scope
  --> 🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️unit/🦀️.rs:49:16
   |
49 |     assert_eq!(Wfc3dBuilderConstruction::from_binary(&bytes).expect("binary builds").build().expect("no diagnostics"), document);
   |                ^^^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `Wfc3dBuilderConstruction`
   |
help: consider importing this struct through its public re-export
   |
 4 + use crate::Wfc3dBuilderConstruction;
   |

error[E0433]: cannot find type `Wfc3dBuilderConstruction` in this scope
  --> 🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️unit/🦀️.rs:50:16
   |
```

## complementary-native-current.log

```text
error[E0433]: cannot find type `Wfc3dBuilderConstruction` in this scope
  --> 🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️unit/🦀️.rs:50:16
   |
50 |     assert_eq!(Wfc3dBuilderConstruction::empty().build().expect("empty builds"), Wfc3dSnapshot::default());
   |                ^^^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `Wfc3dBuilderConstruction`
   |
help: consider importing this struct through its public re-export
   |
 4 + use crate::Wfc3dBuilderConstruction;
   |

error[E0433]: cannot find type `Wfc3dAnalyzerAnalysis` in this scope
  --> 🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️unit/🦀️.rs:57:20
   |
```

## complementary-native-current.log

```text
error[E0433]: cannot find type `Wfc3dAnalyzerAnalysis` in this scope
  --> 🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️unit/🦀️.rs:57:20
   |
57 |     let analysis = Wfc3dAnalyzerAnalysis::analyze(&[AnalyzeSource::Text(&text)]);
   |                    ^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `Wfc3dAnalyzerAnalysis`
   |
help: consider importing this struct through its public re-export
   |
 4 + use crate::Wfc3dAnalyzerAnalysis;
   |

error[E0433]: cannot find type `Wfc3dAnalyzerAnalysis` in this scope
  --> 🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️unit/🦀️.rs:60:20
   |
```

## complementary-native-current.log

```text
error[E0433]: cannot find type `Wfc3dAnalyzerAnalysis` in this scope
  --> 🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️unit/🦀️.rs:60:20
   |
60 |     let analysis = Wfc3dAnalyzerAnalysis::analyze(&[AnalyzeSource::Binary(&bytes)]);
   |                    ^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `Wfc3dAnalyzerAnalysis`
   |
help: consider importing this struct through its public re-export
   |
 4 + use crate::Wfc3dAnalyzerAnalysis;
   |

error[E0433]: cannot find type `Wfc3dAnalyzerAnalysis` in this scope
  --> 🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️unit/🦀️.rs:62:16
   |
```

## complementary-native-current.log

```text
error[E0433]: cannot find type `Wfc3dAnalyzerAnalysis` in this scope
  --> 🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️unit/🦀️.rs:62:16
   |
62 |     assert_eq!(Wfc3dAnalyzerAnalysis::DIALECT.artifact_kind, "s.wfc.wfc3d");
   |                ^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `Wfc3dAnalyzerAnalysis`
   |
help: consider importing this struct through its public re-export
   |
 4 + use crate::Wfc3dAnalyzerAnalysis;
   |

error[E0433]: cannot find type `Wfc3dAnalyzerAnalysis` in this scope
  --> 🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️unit/🦀️.rs:69:20
   |
```

## complementary-native-current.log

```text
error[E0433]: cannot find type `Wfc3dAnalyzerAnalysis` in this scope
  --> 🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️unit/🦀️.rs:69:20
   |
69 |     let analysis = Wfc3dAnalyzerAnalysis::analyze(&[AnalyzeSource::Text("this is not a wfc3d document {{{")]);
   |                    ^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `Wfc3dAnalyzerAnalysis`
   |
help: consider importing this struct through its public re-export
   |
 4 + use crate::Wfc3dAnalyzerAnalysis;
   |

error[E0425]: cannot find function `solve_with_job` in this scope
   --> 🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:127:5
    |
```

## complementary-native-current.log

```text
error[E0425]: cannot find function `solve_with_job` in this scope
   --> 🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:127:5
    |
127 |     solve_with_job(snapshot).map(|commit| commit.assignments).unwrap_or_default()
    |     ^^^^^^^^^^^^^^ not found in this scope
    |
help: consider importing this function through its public re-export
    |
 19 + use crate::solve_with_job;
    |

warning: unused import: `ArtifactAnalysis`
 --> 🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️unit/🦀️.rs:6:45
  |
```

## complementary-native-current.log

```text
error[E0425]: cannot find value `PROBE_TOL` in this scope
   --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/../../📐️brep/💡️queries/✅validation/🦀️.rs:235:76
    |
235 | ...                   if midpoint.distance(other_midpoint) < PROBE_TOL {
    |                                                              ^^^^^^^^^ not found in this scope

warning: unnecessary qualification
   --> /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/../../🥽️mesh/🦀️.rs:225:21
    |
225 |     pub attributes: std::collections::BTreeMap<String,MeshAttribute>,
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: requested on the command line with `-W unused-qualifications`
help: remove the unnecessary path segments
```

## complementary-native-current.log

```text
error[E0277]: the trait bound `semio_framework_artifact_reference::ArtifactRef: serde::Serialize` is not satisfied
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:13490:39
      |
13490 |     #[derive(Clone, Debug, PartialEq, Serialize, ToValue, FromValue)]
      |                                       ^^^^^^^^^ the trait `component::sqlite_wire::_::_serde::Serialize` is not implemented for `semio_framework_artifact_reference::ArtifactRef`
13491 |     pub struct ChildEmitGenesis {
13492 |         pub reference: ArtifactRef,
      |         -------------------------- required by a bound introduced by this call
      |
      = note: for local types consider adding `#[derive(serde::Serialize)]` to your `semio_framework_artifact_reference::ArtifactRef` type
      = note: for types from other crates check whether the crate offers a `serde` feature flag
      = help: the following other types implement trait `component::sqlite_wire::_::_serde::Serialize`:
                &'a T
                &'a mut T
```

## complementary-native-current.log

```text
error[E0432]: unresolved import `semio_framework_artifact_infinite_dag::dag_host_snapshot_to_wire_literal`
 --> 🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧬️compiled/🦀️.rs:8:78
  |
8 | use semio_framework_artifact_infinite_dag::{dag_host_snapshot_from_document, dag_host_snapshot_to_wire_literal, DagCamera};
  |                                                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `dag_host_snapshot_to_wire_literal` in the root

error[E0432]: unresolved import `replace_config`
 --> 🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🚪️io/💾️binary/🧬️mutations/🦀️.rs:8:5
  |
8 | use replace_config::ReplaceConfig;
  |     ^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `replace_config`
  |
  = help: if you wanted to use a crate named `replace_config`, use `cargo add replace_config` to add it to your `Cargo.toml`

```

## complementary-native-current.log

```text
error[E0432]: unresolved import `replace_config`
 --> 🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🚪️io/💾️binary/🧬️mutations/🦀️.rs:8:5
  |
8 | use replace_config::ReplaceConfig;
  |     ^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `replace_config`
  |
  = help: if you wanted to use a crate named `replace_config`, use `cargo add replace_config` to add it to your `Cargo.toml`

error[E0432]: unresolved import `change_camera`
 --> 🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🚪️io/💾️binary/🧬️mutations/🦀️.rs:9:5
  |
9 | use change_camera::ChangeCamera;
  |     ^^^^^^^^^^^^^ use of unresolved module or unlinked crate `change_camera`
  |
```

## complementary-native-current.log

```text
error[E0432]: unresolved import `change_camera`
 --> 🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🚪️io/💾️binary/🧬️mutations/🦀️.rs:9:5
  |
9 | use change_camera::ChangeCamera;
  |     ^^^^^^^^^^^^^ use of unresolved module or unlinked crate `change_camera`
  |
  = help: if you wanted to use a crate named `change_camera`, use `cargo add change_camera` to add it to your `Cargo.toml`

error[E0432]: unresolved import `replace_config`
 --> 🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🚪️io/📝️text/🧬️mutations/🦀️.rs:8:5
  |
8 | use replace_config::ReplaceConfig;
  |     ^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `replace_config`
  |
```

## complementary-native-current.log

```text
error[E0432]: unresolved import `replace_config`
 --> 🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🚪️io/📝️text/🧬️mutations/🦀️.rs:8:5
  |
8 | use replace_config::ReplaceConfig;
  |     ^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `replace_config`
  |
  = help: if you wanted to use a crate named `replace_config`, use `cargo add replace_config` to add it to your `Cargo.toml`

error[E0432]: unresolved import `change_camera`
 --> 🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🚪️io/📝️text/🧬️mutations/🦀️.rs:9:5
  |
9 | use change_camera::ChangeCamera;
  |     ^^^^^^^^^^^^^ use of unresolved module or unlinked crate `change_camera`
  |
```

## complementary-native-current.log

```text
error[E0432]: unresolved import `change_camera`
 --> 🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🚪️io/📝️text/🧬️mutations/🦀️.rs:9:5
  |
9 | use change_camera::ChangeCamera;
  |     ^^^^^^^^^^^^^ use of unresolved module or unlinked crate `change_camera`
  |
  = help: if you wanted to use a crate named `change_camera`, use `cargo add change_camera` to add it to your `Cargo.toml`

error[E0432]: unresolved import `replace_config`
  --> 🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🚪️io/📝️text/🧬️mutations/🦀️.rs:38:5
   |
38 | use replace_config::ReplaceConfig;
   |     ^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `replace_config`
   |
```

## complementary-native-current.log

```text
error[E0432]: unresolved import `replace_config`
  --> 🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🚪️io/📝️text/🧬️mutations/🦀️.rs:38:5
   |
38 | use replace_config::ReplaceConfig;
   |     ^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `replace_config`
   |
   = help: if you wanted to use a crate named `replace_config`, use `cargo add replace_config` to add it to your `Cargo.toml`

error[E0432]: unresolved import `change_camera`
  --> 🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🚪️io/📝️text/🧬️mutations/🦀️.rs:39:5
   |
39 | use change_camera::ChangeCamera;
   |     ^^^^^^^^^^^^^ use of unresolved module or unlinked crate `change_camera`
   |
```

## complementary-native-current.log

```text
error[E0432]: unresolved import `change_camera`
  --> 🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🚪️io/📝️text/🧬️mutations/🦀️.rs:39:5
   |
39 | use change_camera::ChangeCamera;
   |     ^^^^^^^^^^^^^ use of unresolved module or unlinked crate `change_camera`
   |
   = help: if you wanted to use a crate named `change_camera`, use `cargo add change_camera` to add it to your `Cargo.toml`

error[E0432]: unresolved import `replace_presence`
 --> 🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🚪️io/💾️binary/🧬️mutations/🦀️.rs:8:5
  |
8 | use replace_presence::ReplacePresence;
  |     ^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `replace_presence`
  |
```

## complementary-native-current.log

```text
error[E0432]: unresolved import `replace_presence`
 --> 🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🚪️io/💾️binary/🧬️mutations/🦀️.rs:8:5
  |
8 | use replace_presence::ReplacePresence;
  |     ^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `replace_presence`
  |
  = help: if you wanted to use a crate named `replace_presence`, use `cargo add replace_presence` to add it to your `Cargo.toml`

error[E0432]: unresolved import `replace_presence`
 --> 🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🚪️io/📝️text/🧬️mutations/🦀️.rs:8:5
  |
8 | use replace_presence::ReplacePresence;
  |     ^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `replace_presence`
  |
```

## complementary-native-current.log

```text
error[E0432]: unresolved import `replace_presence`
 --> 🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🚪️io/📝️text/🧬️mutations/🦀️.rs:8:5
  |
8 | use replace_presence::ReplacePresence;
  |     ^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `replace_presence`
  |
  = help: if you wanted to use a crate named `replace_presence`, use `cargo add replace_presence` to add it to your `Cargo.toml`

error[E0432]: unresolved import `replace_presence`
  --> 🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🚪️io/📝️text/🧬️mutations/🦀️.rs:37:5
   |
37 | use replace_presence::ReplacePresence;
   |     ^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `replace_presence`
   |
```

## complementary-native-current.log

```text
error[E0432]: unresolved import `replace_presence`
  --> 🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🚪️io/📝️text/🧬️mutations/🦀️.rs:37:5
   |
37 | use replace_presence::ReplacePresence;
   |     ^^^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `replace_presence`
   |
   = help: if you wanted to use a crate named `replace_presence`, use `cargo add replace_presence` to add it to your `Cargo.toml`

error[E0603]: struct import `DagSnapshot` is private
   --> 🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/../.././🔨️modules/🏠️host/🧰️owned/🦀️.rs:5:64
    |
  5 | use crate::standards::v1::subsets::any::io::binary::snapshot::{DagSnapshot};
    |                                                                ^^^^^^^^^^^ private struct import
    |
```

## complementary-native-current.log

```text
error[E0603]: struct import `DagSnapshot` is private
   --> 🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/../.././🔨️modules/🏠️host/🧰️owned/🦀️.rs:5:64
    |
  5 | use crate::standards::v1::subsets::any::io::binary::snapshot::{DagSnapshot};
    |                                                                ^^^^^^^^^^^ private struct import
    |
note: the struct import `DagSnapshot` is defined here...
   --> 🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🦀️.rs:2:5
    |
  2 | use crate::DagSnapshot;
    |     ^^^^^^^^^^^^^^^^^^
note: ...and refers to the struct import `DagSnapshot` which is defined here...
   --> 🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/../../🦀️.rs:538:9
    |
```

## complementary-native-current.log

```text
error[E0433]: cannot find module or crate `member_group` in this scope
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:26791:62
      |
26791 | ...   publication.publication.group_preparation = Some(member_group::MemberGroupPreparation::new(visibility, publication.group_hi...
      |                                                        ^^^^^^^^^^^^ use of unresolved module or unlinked crate `member_group`
      |
      = help: if you wanted to use a crate named `member_group`, use `cargo add member_group` to add it to your `Cargo.toml`

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/././../../🔨️modules/🗣️dsl/👪️family/📊️sheet/🦀️.rs:55:9
   |
55 |         semio_framework_dsl_record::ExprValue::Num(v) => Ok(*v),
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
```

## complementary-native-current.log

```text
error[E0609]: no field `group_preparation` on type `ArtifactStoreBatchPublication<P, Mutation>`
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:26790:36
      |
26790 |         if publication.publication.group_preparation.is_none() {
      |                                    ^^^^^^^^^^^^^^^^^ unknown field
      |
      = note: available fields are: `operation`, `expected_generation`, `expected_revision`, `lane`, `footprint` ... and 21 others

error[E0609]: no field `group_preparation` on type `ArtifactStoreBatchPublication<P, Mutation>`
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:26791:37
      |
26791 | ...   publication.publication.group_preparation = Some(member_group::MemberGroupPreparation::new(visibility, publication.group_hi...
      |                               ^^^^^^^^^^^^^^^^^ unknown field
      |
```

## complementary-native-current.log

```text
error[E0609]: no field `group_preparation` on type `ArtifactStoreBatchPublication<P, Mutation>`
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:26791:37
      |
26791 | ...   publication.publication.group_preparation = Some(member_group::MemberGroupPreparation::new(visibility, publication.group_hi...
      |                               ^^^^^^^^^^^^^^^^^ unknown field
      |
      = note: available fields are: `operation`, `expected_generation`, `expected_revision`, `lane`, `footprint` ... and 21 others

error[E0599]: no method named `stage_apply_batch_group` found for mutable reference `&mut os_store::component::ArtifactStore<P, Mutation>` in the current scope
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:26793:14
      |
26793 |         self.stage_apply_batch_group(&mut publication.publication, visibility, grant)
      |              ^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&mut os_store::component::ArtifactStore<P, Mutation>`

```

## complementary-native-current.log

```text
error[E0599]: no method named `stage_apply_batch_group` found for mutable reference `&mut os_store::component::ArtifactStore<P, Mutation>` in the current scope
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:26793:14
      |
26793 |         self.stage_apply_batch_group(&mut publication.publication, visibility, grant)
      |              ^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&mut os_store::component::ArtifactStore<P, Mutation>`

error[E0599]: no method named `adopt_apply_batch_group` found for mutable reference `&mut os_store::component::ArtifactStore<P, Mutation>` in the current scope
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:26799:14
      |
26799 |         self.adopt_apply_batch_group(&mut publication.publication, visibility, grant)
      |              ^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&mut os_store::component::ArtifactStore<P, Mutation>`

error[E0609]: no field `group_preparation` on type `ArtifactStoreBatchPublication<P, Mutation>`
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:26813:36
```

## complementary-native-current.log

```text
error[E0599]: no method named `adopt_apply_batch_group` found for mutable reference `&mut os_store::component::ArtifactStore<P, Mutation>` in the current scope
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:26799:14
      |
26799 |         self.adopt_apply_batch_group(&mut publication.publication, visibility, grant)
      |              ^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&mut os_store::component::ArtifactStore<P, Mutation>`

error[E0609]: no field `group_preparation` on type `ArtifactStoreBatchPublication<P, Mutation>`
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:26813:36
      |
26813 |         if publication.publication.group_preparation.is_some() {
      |                                    ^^^^^^^^^^^^^^^^^ unknown field
      |
      = note: available fields are: `operation`, `expected_generation`, `expected_revision`, `lane`, `footprint` ... and 21 others

```

## complementary-native-current.log

```text
error[E0609]: no field `group_preparation` on type `ArtifactStoreBatchPublication<P, Mutation>`
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:26813:36
      |
26813 |         if publication.publication.group_preparation.is_some() {
      |                                    ^^^^^^^^^^^^^^^^^ unknown field
      |
      = note: available fields are: `operation`, `expected_generation`, `expected_revision`, `lane`, `footprint` ... and 21 others

error[E0599]: no method named `abort_apply_batch_group` found for mutable reference `&mut os_store::component::ArtifactStore<P, Mutation>` in the current scope
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:26814:25
      |
26814 | ...   return self.abort_apply_batch_group(&mut publication.publication, grant).map(|step| if step == SnapshotRetirementStep::Comp...
      |                   ^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&mut os_store::component::ArtifactStore<P, Mutation>`

```

## complementary-native-current.log

```text
error[E0599]: no method named `abort_apply_batch_group` found for mutable reference `&mut os_store::component::ArtifactStore<P, Mutation>` in the current scope
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:26814:25
      |
26814 | ...   return self.abort_apply_batch_group(&mut publication.publication, grant).map(|step| if step == SnapshotRetirementStep::Comp...
      |                   ^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&mut os_store::component::ArtifactStore<P, Mutation>`

error[E0599]: no method named `next_group_byte_demand` found for struct `ArtifactStoreBatchPublication<P, Mutation>` in the current scope
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:17507:66
      |
17507 |     fn next_group_byte_demand(&self) -> usize { self.publication.next_group_byte_demand() }
      |                                                                  ^^^^^^^^^^^^^^^^^^^^^^
...
17694 | pub struct ArtifactStoreBatchPublication<P, Mutation> {
      | ----------------------------------------------------- method `next_group_byte_demand` not found for this struct
```

## complementary-native-current.log

```text
error[E0599]: no method named `next_group_byte_demand` found for struct `ArtifactStoreBatchPublication<P, Mutation>` in the current scope
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:17507:66
      |
17507 |     fn next_group_byte_demand(&self) -> usize { self.publication.next_group_byte_demand() }
      |                                                                  ^^^^^^^^^^^^^^^^^^^^^^
...
17694 | pub struct ArtifactStoreBatchPublication<P, Mutation> {
      | ----------------------------------------------------- method `next_group_byte_demand` not found for this struct
      |
      = help: items from traits can only be used if the trait is implemented and in scope
note: `os_store::component::ErasedMemberStoreOneItemPublication` defines an item `next_group_byte_demand`, perhaps you need to implement it
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:17474:1
      |
17474 | pub trait ErasedMemberStoreOneItemPublication: Send {
```

## complementary-native-current.log

```text
error[E0425]: cannot find function `stage_prebuilt_batch_root` in module `durable_group`
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:127:53
    |
127 | ...   Phase::Transfer => { durable_group::stage_prebuilt_batch_root(self, publication, &mut group)?; group.phase = Phase::Staged; }
    |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `durable_group`

error[E0603]: function `adopt_staged_store_member` is private
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:142:24
    |
142 |         durable_group::adopt_staged_store_member(self, visibility).map_err(|error| error.to_string())?;
    |                        ^^^^^^^^^^^^^^^^^^^^^^^^^ private function
    |
note: the function `adopt_staged_store_member` is defined here
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:685:1
```

## complementary-native-current.log

```text
error[E0603]: function `adopt_staged_store_member` is private
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:142:24
    |
142 |         durable_group::adopt_staged_store_member(self, visibility).map_err(|error| error.to_string())?;
    |                        ^^^^^^^^^^^^^^^^^^^^^^^^^ private function
    |
note: the function `adopt_staged_store_member` is defined here
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:685:1
    |
685 | / fn adopt_staged_store_member<P, Mutation>(store: &mut ArtifactStore<P, Mutation>, visibility: &Arc<crate::os_vcs::ArtifactGroupVi...
686 | | where
687 | |     P: ArtifactPack + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
688 | |     Mutation: StoreMutation<P> + Clone + ValueToValue + ValueFromValue + Send + 'static,
...   |
```

## complementary-native-current.log

```text
error[E0603]: function `abort_staged_store_member` is private
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:158:28
    |
158 |             durable_group::abort_staged_store_member(self, &group.visibility).map_err(|error| fault(&error.to_string()))?;
    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^ private function
    |
note: the function `abort_staged_store_member` is defined here
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:634:1
    |
634 | / fn abort_staged_store_member<P, Mutation>(store: &mut ArtifactStore<P, Mutation>, visibility: &Arc<crate::os_vcs::ArtifactGroupVi...
635 | | where
636 | |     P: ArtifactPack + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
637 | |     Mutation: StoreMutation<P> + Clone + ValueToValue + ValueFromValue + Send + 'static,
...   |
```

## complementary-native-current.log

```text
error[E0277]: the trait bound `P: Clone` is not satisfied
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:33:37
      |
   33 |     fn demand<P, Mu>(&self, store: &ArtifactStore<P, Mu>, stage: Option<&ArtifactStoreBatchStage<P, Mu>>) -> usize {
      |                                     ^^^^^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `P`
      |
note: required by a bound in `os_store::component::ArtifactStore`
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:17981:8
      |
17979 | pub struct ArtifactStore<P, Mutation>
      |            ------------- required by a bound in this struct
17980 | where
17981 |     P: Clone + ToValue + FromValue,
      |        ^^^^^ required by this bound in `ArtifactStore`
```

## complementary-native-current.log

```text
error[E0277]: the trait bound `P: semio_framework_value::ToValue` is not satisfied
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:33:37
      |
   33 |     fn demand<P, Mu>(&self, store: &ArtifactStore<P, Mu>, stage: Option<&ArtifactStoreBatchStage<P, Mu>>) -> usize {
      |                                     ^^^^^^^^^^^^^^^^^^^^ the trait `semio_framework_value::ToValue` is not implemented for `P`
      |
note: required by a bound in `os_store::component::ArtifactStore`
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:17981:16
      |
17979 | pub struct ArtifactStore<P, Mutation>
      |            ------------- required by a bound in this struct
17980 | where
17981 |     P: Clone + ToValue + FromValue,
      |                ^^^^^^^ required by this bound in `ArtifactStore`
```

## complementary-native-current.log

```text
error[E0277]: the trait bound `P: semio_framework_value::FromValue` is not satisfied
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:33:37
      |
   33 |     fn demand<P, Mu>(&self, store: &ArtifactStore<P, Mu>, stage: Option<&ArtifactStoreBatchStage<P, Mu>>) -> usize {
      |                                     ^^^^^^^^^^^^^^^^^^^^ the trait `semio_framework_value::FromValue` is not implemented for `P`
      |
note: required by a bound in `os_store::component::ArtifactStore`
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:17981:26
      |
17979 | pub struct ArtifactStore<P, Mutation>
      |            ------------- required by a bound in this struct
17980 | where
17981 |     P: Clone + ToValue + FromValue,
      |                          ^^^^^^^^^ required by this bound in `ArtifactStore`
```

## complementary-native-current.log

```text
error[E0277]: the trait bound `Mu: protocol::Mutation<P>` is not satisfied
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:33:37
      |
   33 |     fn demand<P, Mu>(&self, store: &ArtifactStore<P, Mu>, stage: Option<&ArtifactStoreBatchStage<P, Mu>>) -> usize {
      |                                     ^^^^^^^^^^^^^^^^^^^^ the trait `protocol::Mutation<P>` is not implemented for `Mu`
      |
note: required by a bound in `os_store::component::ArtifactStore`
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:17982:45
      |
17979 | pub struct ArtifactStore<P, Mutation>
      |            ------------- required by a bound in this struct
...
17982 |     Mutation: Clone + ToValue + FromValue + self::Mutation<P>,
      |                                             ^^^^^^^^^^^^^^^^^ required by this bound in `ArtifactStore`
```

## complementary-native-current.log

```text
error[E0277]: the trait bound `P: Clone` is not satisfied
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:34:37
      |
   34 |     fn demand<P, Mu>(&self, store: &ArtifactStore<P, Mu>, stage: Option<&ArtifactStoreBatchStage<P, Mu>>) -> usize {
      |                                     ^^^^^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `P`
      |
note: required by a bound in `os_store::component::ArtifactStore`
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:17981:8
      |
17979 | pub struct ArtifactStore<P, Mutation>
      |            ------------- required by a bound in this struct
17980 | where
17981 |     P: Clone + ToValue + FromValue,
      |        ^^^^^ required by this bound in `ArtifactStore`
```

## complementary-native-current.log

```text
error[E0277]: the trait bound `P: semio_framework_value::ToValue` is not satisfied
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:34:37
      |
   34 |     fn demand<P, Mu>(&self, store: &ArtifactStore<P, Mu>, stage: Option<&ArtifactStoreBatchStage<P, Mu>>) -> usize {
      |                                     ^^^^^^^^^^^^^^^^^^^^ the trait `semio_framework_value::ToValue` is not implemented for `P`
      |
note: required by a bound in `os_store::component::ArtifactStore`
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:17981:16
      |
17979 | pub struct ArtifactStore<P, Mutation>
      |            ------------- required by a bound in this struct
17980 | where
17981 |     P: Clone + ToValue + FromValue,
      |                ^^^^^^^ required by this bound in `ArtifactStore`
```

## complementary-native-current.log

```text
error[E0277]: the trait bound `P: semio_framework_value::FromValue` is not satisfied
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:34:37
      |
   34 |     fn demand<P, Mu>(&self, store: &ArtifactStore<P, Mu>, stage: Option<&ArtifactStoreBatchStage<P, Mu>>) -> usize {
      |                                     ^^^^^^^^^^^^^^^^^^^^ the trait `semio_framework_value::FromValue` is not implemented for `P`
      |
note: required by a bound in `os_store::component::ArtifactStore`
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:17981:26
      |
17979 | pub struct ArtifactStore<P, Mutation>
      |            ------------- required by a bound in this struct
17980 | where
17981 |     P: Clone + ToValue + FromValue,
      |                          ^^^^^^^^^ required by this bound in `ArtifactStore`
```

## complementary-native-current.log

```text
error[E0277]: the trait bound `Mu: protocol::Mutation<P>` is not satisfied
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:34:37
      |
   34 |     fn demand<P, Mu>(&self, store: &ArtifactStore<P, Mu>, stage: Option<&ArtifactStoreBatchStage<P, Mu>>) -> usize {
      |                                     ^^^^^^^^^^^^^^^^^^^^ the trait `protocol::Mutation<P>` is not implemented for `Mu`
      |
note: required by a bound in `os_store::component::ArtifactStore`
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:17982:45
      |
17979 | pub struct ArtifactStore<P, Mutation>
      |            ------------- required by a bound in this struct
...
17982 |     Mutation: Clone + ToValue + FromValue + self::Mutation<P>,
      |                                             ^^^^^^^^^^^^^^^^^ required by this bound in `ArtifactStore`
```

## complementary-native-current.log

```text
error[E0283]: type annotations needed
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:101:138
     |
 101 | ...umulator.indexed_edits.range((Excluded(key), Unbounded)).next(), None => self.revision_accumulator.indexed_edits.first_key_valu...
     |                           ^^^^^ cannot infer type of the type parameter `T` declared on the method `range`
     |
     = note: the type must implement `Ord`
note: required by a bound in `std::collections::BTreeMap::<K, V, A>::range`
    --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/collections/btree/map.rs:1431:12
     |
1429 |     pub fn range<T: ?Sized, R>(&self, range: R) -> Range<'_, K, V>
     |            ----- required by a bound in this associated function
1430 |     where
1431 |         T: Ord,
```

## complementary-native-current.log

```text
error[E0283]: type annotations needed
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:103:138
     |
 103 | ...umulator.indexed_edits.range((Excluded(key), Unbounded)).next(), None => self.revision_accumulator.indexed_edits.first_key_valu...
     |                           ^^^^^ cannot infer type of the type parameter `T` declared on the method `range`
     |
     = note: the type must implement `Ord`
note: required by a bound in `std::collections::BTreeMap::<K, V, A>::range`
    --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/collections/btree/map.rs:1431:12
     |
1429 |     pub fn range<T: ?Sized, R>(&self, range: R) -> Range<'_, K, V>
     |            ----- required by a bound in this associated function
1430 |     where
1431 |         T: Ord,
```

## complementary-native-current.log

```text
error[E0433]: cannot find `protocol` in `super`
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📨️emission/📦️owned/🦀️.rs:122:36
    |
122 |     pub transaction: Option<super::protocol::TransactionRef>,
    |                                    ^^^^^^^^ could not find `protocol` in `super`

warning: unnecessary qualification
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/././../../🔨️modules/🗣️dsl/👪️family/📊️sheet/🦀️.rs:55:9
   |
55 |         semio_framework_dsl_record::ExprValue::Num(v) => Ok(*v),
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: requested on the command line with `-W unused-qualifications`
help: remove the unnecessary path segments
```

## complementary-native-current.log

```text
error[E0283]: type annotations needed
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:104:138
     |
 104 | ...umulator.indexed_edits.range((Excluded(key), Unbounded)).next(), None => self.revision_accumulator.indexed_edits.first_key_valu...
     |                           ^^^^^ cannot infer type of the type parameter `T` declared on the method `range`
     |
     = note: the type must implement `Ord`
note: required by a bound in `std::collections::BTreeMap::<K, V, A>::range`
    --> /Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/collections/btree/map.rs:1431:12
     |
1429 |     pub fn range<T: ?Sized, R>(&self, range: R) -> Range<'_, K, V>
     |            ----- required by a bound in this associated function
1430 |     where
1431 |         T: Ord,
```

## complementary-native-current.log

```text
error[E0624]: method `stage_apply_batch_group` is private
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:26804:14
      |
26804 |         self.stage_apply_batch_group(&mut publication.publication, visibility, grant)
      |              ^^^^^^^^^^^^^^^^^^^^^^^ private method
      |
     ::: /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:36:5
      |
   36 |     fn stage_apply_batch_group(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mu>, visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> {
      |     ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ private method defined here

error[E0624]: method `adopt_apply_batch_group` is private
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:26810:14
      |
```

## complementary-native-current.log

```text
error[E0624]: method `adopt_apply_batch_group` is private
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:26810:14
      |
26810 |         self.adopt_apply_batch_group(&mut publication.publication, visibility, grant)
      |              ^^^^^^^^^^^^^^^^^^^^^^^ private method
      |
     ::: /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:115:5
      |
  115 |     fn adopt_apply_batch_group(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mu>, visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> {
      |     ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ private method defined here

error[E0624]: method `abort_apply_batch_group` is private
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:26825:25
      |
```

## complementary-native-current.log

```text
error[E0624]: method `abort_apply_batch_group` is private
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:26825:25
      |
26825 | ...   return self.abort_apply_batch_group(&mut publication.publication, grant).map(|step| if step == SnapshotRetirementStep::Comp...
      |                   ^^^^^^^^^^^^^^^^^^^^^^^ private method
      |
     ::: /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:136:5
      |
  136 |     fn abort_apply_batch_group(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mu>, grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, ValueError> {
      |     ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- private method defined here

[cargo:build] running elapsedMs=30012
[artifact-rust:semio-s-artifact-layout-layout:test] running elapsedMs=43861
error[E0599]: no method named `completed_items` found for reference `&MemberGroupPreparation` in the current scope
```

## complementary-native-current.log

```text
error[E0599]: no method named `completed_items` found for reference `&MemberGroupPreparation` in the current scope
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:17795:161
      |
17795 | ...s_ref().map_or(0, |group| group.completed_items())),
      |                                    ^^^^^^^^^^^^^^^ method not found in `&MemberGroupPreparation`

warning: `semio-framework-graph` (lib) generated 18 warnings (run `cargo fix --lib -p semio-framework-graph` to apply 16 suggestions)
   Compiling wasmtime v47.0.3
error[E0616]: field `history` of struct `MemberGroupPreparation` is private
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:635:25
    |
635 |     let history = group.history.take().ok_or_else(|| "batch group final transfer lost its history reservation".to_string())?;
    |                         ^^^^^^^ private field

```

## complementary-native-current.log

```text
error[E0616]: field `history` of struct `MemberGroupPreparation` is private
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:635:25
    |
635 |     let history = group.history.take().ok_or_else(|| "batch group final transfer lost its history reservation".to_string())?;
    |                         ^^^^^^^ private field

error[E0616]: field `displaced` of struct `MemberGroupPreparation` is private
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:636:31
    |
636 |     let mut displaced = group.displaced.take().expect("copied group admits its displaced owner catalog");
    |                               ^^^^^^^^^ private field

error[E0609]: no field `cursor_owners` on type `&mut MemberGroupPreparation`
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:637:24
```

## complementary-native-current.log

```text
error[E0616]: field `displaced` of struct `MemberGroupPreparation` is private
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:636:31
    |
636 |     let mut displaced = group.displaced.take().expect("copied group admits its displaced owner catalog");
    |                               ^^^^^^^^^ private field

error[E0609]: no field `cursor_owners` on type `&mut MemberGroupPreparation`
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:637:24
    |
637 |     let cursor = group.cursor_owners.take().expect("copied group retains its cursor catalogs");
    |                        ^^^^^^^^^^^^^ unknown field

error[E0616]: field `applied` of struct `MemberGroupPreparation` is private
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:638:25
```

## complementary-native-current.log

```text
error[E0609]: no field `cursor_owners` on type `&mut MemberGroupPreparation`
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:637:24
    |
637 |     let cursor = group.cursor_owners.take().expect("copied group retains its cursor catalogs");
    |                        ^^^^^^^^^^^^^ unknown field

error[E0616]: field `applied` of struct `MemberGroupPreparation` is private
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:638:25
    |
638 |     let applied = group.applied.take().expect("copied group retains its applied catalog");
    |                         ^^^^^^^ private field

error[E0616]: field `revision` of struct `MemberGroupPreparation` is private
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:639:30
```

## complementary-native-current.log

```text
error[E0616]: field `applied` of struct `MemberGroupPreparation` is private
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:638:25
    |
638 |     let applied = group.applied.take().expect("copied group retains its applied catalog");
    |                         ^^^^^^^ private field

error[E0616]: field `revision` of struct `MemberGroupPreparation` is private
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:639:30
    |
639 |     let mut revision = group.revision.take().expect("copied group retains its complete revision and indexes");
    |                              ^^^^^^^^ private field

error[E0599]: no method named `visibility` found for mutable reference `&mut MemberGroupPreparation` in the current scope
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:642:28
```

## complementary-native-current.log

```text
error[E0616]: field `revision` of struct `MemberGroupPreparation` is private
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:639:30
    |
639 |     let mut revision = group.revision.take().expect("copied group retains its complete revision and indexes");
    |                              ^^^^^^^^ private field

error[E0599]: no method named `visibility` found for mutable reference `&mut MemberGroupPreparation` in the current scope
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:642:28
    |
642 |     let visibility = group.visibility();
    |                            ^^^^^^^^^^ private field, not a method

warning: unused variable: `offset`
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🎒️pack/🌱️value/🦀️.rs:2206:17
```

## complementary-native-current.log

```text
error[E0599]: no method named `visibility` found for mutable reference `&mut MemberGroupPreparation` in the current scope
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:642:28
    |
642 |     let visibility = group.visibility();
    |                            ^^^^^^^^^^ private field, not a method

warning: unused variable: `offset`
    --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🎒️pack/🌱️value/🦀️.rs:2206:17
     |
2206 |             let offset = reader.position() as u64;
     |                 ^^^^^^ help: if this is intentional, prefix it with an underscore: `_offset`
     |
     = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

```

## complementary-native-current.log

```text
error[E0308]: mismatched types
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:17795:155
      |
17795 | ...paration.as_ref().map_or(0, |group| group.completed_items())),
      |                                        ^^^^^^^^^^^^^^^^^^^^^^^ expected `u32`, found `usize`
      |
help: you can convert a `usize` to a `u32` and panic if the converted value doesn't fit
      |
17795 |             completed_items: staged.completed_items.saturating_add(item.completed_items).saturating_add(self.group_preparation.as_ref().map_or(0, |group| group.completed_items().try_into().unwrap())),
      |                                                                                                                                                                                  ++++++++++++++++++++

warning: unused variable: `local_actor`
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:20664:75
      |
```

## complementary-native-current.log

```text
error[E0308]: mismatched types
   --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs:117:179
    |
117 | ...eparation.as_ref().map_or(0, |group| group.completed_items()));
    |                                         ^^^^^^^^^^^^^^^^^^^^^^^ expected `u32`, found `usize`
    |
help: you can convert a `usize` to a `u32` and panic if the converted value doesn't fit
    |
117 |         publication.retained_checkpoint.completed_items = publication.retained_checkpoint.completed_items.saturating_add(publication.group_preparation.as_ref().map_or(0, |group| group.completed_items().try_into().unwrap()));
    |                                                                                                                                                                                                          ++++++++++++++++++++

warning: unused variable: `local_actor`
     --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:20664:75
      |
```