@capability-shooting-1-mutate
@oracle-shooting-1-python-independent
@comparison-ordered-json-v1
@mutations-shooting-1-any
Feature: Apply every typed SHOOTING mutation to the vocabulary's own committed render scene and against an independent Python implementation
  `s.shooting.shooting` is a semio-NATIVE artifact — the `shooting.shooting.dsl` grammar, with its typed
  table columns and its `deg`-suffixed angle literals, is defined by this repository alone. This case
  carries a recorded no-oracle decision (`shooting-render-scene-mutation-semantics`, in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracle/🔣️.json`) whose third-party survey is
  argued rather than assumed: glTF 2.0, USD and Collada are named, and declined on the one structural
  point that matters here — none of them models a SHOT, and eleven of the thirty-one kinds address
  one. That decision is narrowed to an empty `capabilities` list rather than deleted (it already was,
  by a prior shard of this same ticket), because its own investigation remains the honest record of
  what was checked; a dated note is appended recording that the `asset://` blocker it named is now
  resolved.

  🐍️ `🐍️component.py` beside this file is the second IMPLEMENTATION that decision named as the
  remaining debt: all thirty-one kinds of this vocabulary, written in Python from this subset's own
  committed `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json` document shape and each
  kind's own committed `(mutation, after)` leaf fixture, and from
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/📓️taxonomy.md`'s
  verb table. It imports nothing from the Rust it judges and transliterates none of it. Both
  implementations now read the SAME committed bytes: every kind's own `(mutation, after)` path is a
  declared `asset://` fixture (added to the ONE shared before-document this feature already declared),
  rather than being carried only inline in the `params` column, so the plan pins its digest and a
  Python reference can resolve it. Separately, `identity-round-trip` remains refused for the reason
  this decision already states: this subset's committed snapshot text grammar is the repository-wide
  placeholder `payload = OCTET+`, whose header production declares `"schema" SP "stdio.json"` against
  an artifact whose own first line says otherwise.

  📄️ The base document is real, committed, and is not this case's invention in any part. All thirty-one
  of this vocabulary's per-kind leaf fixtures under
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<kind>/🧪️tests/` commit the SAME
  before-snapshot, byte for byte — SHA-1 `6441b72754e5c649b2b07a2f2b244313467f85a0`, verified across all
  thirty-one copies — and this case reads that one document where the domain already keeps it, at
  `asset://🧬️schema/🧬️mutations/✏️rename-asset/🧪️tests/🏷️renames-asset-hero-to-lead/📸️snapshot/⬅️before/🔣️.json`.
  It carries two assets (one with a quaternion orientation and a non-unit scale, one with neither), two
  saved cameras, two shots (one with a background and a camera reference, one with neither), a full scene
  lighting block and both active-selection ids. Every `params` cell below is likewise the committed
  payload of that kind's OWN leaf fixture, transcribed verbatim and not reworded — this is the one subset
  in this wave whose committed fixtures pin POSITIVE branches rather than guards, because
  `ShootingSnapshot` holds its assets, shots, cameras and scene INLINE and mints no content-addressed
  child handle, so its leaf authors could state a real `➡️after` by hand. All thirty-one were checked
  against the shared before-state and every one of them moves it. The `dir`/`fixture` columns name the
  SAME leaf, now also declared as `asset://` fixtures so the Python reference can resolve them.

  🎞️ The `identity-round-trip` scenario reads a different real committed file on purpose: the plugin's
  own DSL example at
  `asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`.
  That is where the TEXT codec's evidence has to come from — the before-snapshot above is committed as
  JSON and would prove nothing about the handcrafted block/table grammar. The example carries only one
  asset and no saved camera at all, which is exactly why it cannot serve as the mutation base: six kinds
  address the saved-camera collection and would have nothing to address.

  🧬️ The vocabulary is `ShootingMutation`'s thirty-one variants in declaration order and its shape is
  this subset's own. Three id-keyed ORDERED collections each take the per-collection recipe — `assets`
  gets create/delete/rename/change-url/reorder plus three BATCH spatial verbs (`drag`, `rotate`, `scale`
  each carry an `asset_ids` list, because a viewport gesture moves a selection and not a record),
  `shots` gets create/delete/rename/reorder plus one verb per independently editable render field
  (width, height, format, shape) and a camera replacement, `saved-cameras` gets
  create/delete/rename/reorder plus a view replacement — then two active-selection verbs, then one verb
  per scene lighting scalar. There is no `no-mutation` and no `set-snapshot`: whole-document replacement
  is not expressible as an in-history mutation in this generation of the taxonomy and goes through
  `ArtifactStore::reset`. Note the spelling: no `ShootingMutation` payload renames its fields, so every
  field below is snake_case (`new_name`, `asset_ids`, `to_index`) while the variant tag itself is
  camelCase — exactly as each committed leaf fixture is. Every `create-<singular>` kind's `index` field
  is descriptive of authoring intent only — apply is append-only, confirmed against production's own
  docstring on `CreateAsset`/`CreateShot`/`CreateSavedCamera` — which is why every committed `delete-*`
  vector removes the TRAILING member: an append-only re-creation can only land back on the original
  position when that position was last.

  ⚖️ The projection is `(schema, assets, savedCameras, scene, shots, activeShotId, activeAssetId)`. The
  composed `emblem` child handle is deliberately NOT projected: it is a content address for an
  `s.stdio.semio.image` child, no kind of this vocabulary addresses it, and the committed base document
  does not carry one.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the committed render scene and observe it move
    Given the committed before-snapshot every leaf fixture of this vocabulary shares asset://🧬️schema/🧬️mutations/✏️rename-asset/🧪️tests/🏷️renames-asset-hero-to-lead/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-document asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    When the <id> mutation is applied through apply_shooting_mutation
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the resulting projection differs from the base projection, and the two implementations agree
    Examples:
      | id                              | params                                                                                                                                                                            | dir                               | fixture                               |
      | create-asset                    | {"mutation":"createAsset","asset":{"id":"asset-detail","name":"Detail","url":"/mesh/detail.glb","format":"glb","origin":[0.0,4.0,0.0],"orientation":null,"scale":null},"index":0} | ➕️create-asset                    | ➕️appends-asset-detail                  |
      | delete-asset                    | {"mutation":"deleteAsset","id":"asset-prop"}                                                                                                                                      | 🗑️delete-asset                    | 🗑️removes-trailing-asset-prop           |
      | rename-asset                    | {"mutation":"renameAsset","id":"asset-hero","new_name":"Lead"}                                                                                                                    | ✏️rename-asset                    | 🏷️renames-asset-hero-to-lead            |
      | change-asset-url                | {"mutation":"changeAssetUrl","id":"asset-prop","new_url":"/mesh/prop-v2.glb"}                                                                                                     | 🌐️change-asset-url                | 🌐️points-asset-prop-at-v2-mesh          |
      | reorder-assets                  | {"mutation":"reorderAssets","id":"asset-hero","to_index":1}                                                                                                                       | 🔀️reorder-assets                  | 🔀️moves-asset-hero-behind-asset-prop    |
      | drag-assets                     | {"mutation":"dragAssets","asset_ids":["asset-hero","asset-prop","asset-ghost"],"dx":4.0,"dy":-1.0,"dz":0.5}                                                                       | ↔️drag-assets                     | 🚚️offsets-both-assets-and-skips-a-ghost |
      | rotate-assets                   | {"mutation":"rotateAssets","asset_ids":["asset-hero"],"ax":0.0,"ay":0.0,"az":1.0,"angle":1.5}                                                                                     | 🔄️rotate-assets                   | 🔄️spins-asset-hero-about-z              |
      | scale-assets                    | {"mutation":"scaleAssets","asset_ids":["asset-hero"],"sx":2.0,"sy":2.0,"sz":2.0}                                                                                                  | ↕️scale-assets                    | 📏️doubles-asset-hero-scale              |
      | create-shot                     | {"mutation":"createShot","shot":{"id":"shot-macro","label":"Macro","width":128,"height":128,"format":"png","shape":"rectangle"},"index":null}                                     | 📸️create-shot                     | 📸️appends-shot-macro                    |
      | delete-shot                     | {"mutation":"deleteShot","id":"shot-close"}                                                                                                                                       | 🚮️delete-shot                     | 🚫️removes-trailing-shot-close           |
      | rename-shot                     | {"mutation":"renameShot","id":"shot-close","new_label":"Detail"}                                                                                                                  | 🏷️rename-shot                     | 🔤️relabels-shot-close-to-detail         |
      | change-shot-width               | {"mutation":"changeShotWidth","id":"shot-close","new_width":1024}                                                                                                                 | 📏️change-shot-width               | ↔️widens-shot-close-to-1024             |
      | change-shot-height              | {"mutation":"changeShotHeight","id":"shot-close","new_height":768}                                                                                                                | 📐️change-shot-height              | ↕️heightens-shot-close-to-768           |
      | change-shot-format              | {"mutation":"changeShotFormat","id":"shot-wide","new_format":"svg"}                                                                                                               | 🖼️change-shot-format              | 🎨️switches-shot-wide-to-svg             |
      | change-shot-shape               | {"mutation":"changeShotShape","id":"shot-wide","new_shape":"ellipse"}                                                                                                             | ✂️change-shot-shape               | ⭕️rounds-shot-wide-to-ellipse           |
      | reorder-shots                   | {"mutation":"reorderShots","id":"shot-close","to_index":0}                                                                                                                        | 🔃️reorder-shots                   | ⬆️moves-shot-close-to-front             |
      | replace-shot-camera             | {"mutation":"replaceShotCamera","shot_id":"shot-wide","new_camera":{"position":[3.0,-3.0,2.0],"target":[0.0,0.0,0.5],"zoom":1.5,"fov":40.0}}                                      | 📷️replace-shot-camera             | 📷️rewrites-cam-wide-through-shot-wide   |
      | create-saved-camera             | {"mutation":"createSavedCamera","saved_camera":{"id":"cam-top","label":"Top","camera":{"position":[0.0,0.0,20.0],"target":[0.0,0.0,0.0],"zoom":1.0,"fov":50.0}},"index":null}     | 🎥️create-saved-camera             | 🎥️appends-saved-camera-top              |
      | delete-saved-camera             | {"mutation":"deleteSavedCamera","id":"cam-close"}                                                                                                                                 | 🧹️delete-saved-camera             | 🚫️removes-trailing-cam-close            |
      | rename-saved-camera             | {"mutation":"renameSavedCamera","id":"cam-close","new_label":"Tight"}                                                                                                             | 🪪️rename-saved-camera             | 🔤️relabels-cam-close-to-tight           |
      | replace-saved-camera-view       | {"mutation":"replaceSavedCameraView","id":"cam-close","new_camera":{"position":[1.0,-1.0,0.75],"target":[0.0,0.0,1.0],"zoom":4.0,"fov":20.0}}                                     | 🎞️replace-saved-camera-view       | 📍️repositions-cam-close-view            |
      | reorder-saved-cameras           | {"mutation":"reorderSavedCameras","id":"cam-close","to_index":0}                                                                                                                  | 🔁️reorder-saved-cameras           | 🔁️moves-cam-close-to-front              |
      | set-active-shot                 | {"mutation":"setActiveShot","shot_id":"shot-close"}                                                                                                                               | 🎯️set-active-shot                 | 🎯️activates-shot-close                  |
      | set-active-asset                | {"mutation":"setActiveAsset","asset_id":"asset-prop"}                                                                                                                             | 📌️set-active-asset                | 📌️activates-asset-prop                  |
      | change-scene-sun-enabled        | {"mutation":"changeSceneSunEnabled","new_enabled":false}                                                                                                                          | ☀️change-scene-sun-enabled        | ☀️switches-scene-sun-off                |
      | change-scene-sun-azimuth        | {"mutation":"changeSceneSunAzimuth","new_azimuth":315.0}                                                                                                                          | 🧭️change-scene-sun-azimuth        | 🧭️turns-scene-sun-to-315-degrees        |
      | change-scene-sun-elevation      | {"mutation":"changeSceneSunElevation","new_elevation":60.0}                                                                                                                       | 🌅️change-scene-sun-elevation      | 🌅️raises-scene-sun-to-60-degrees        |
      | change-scene-sun-intensity      | {"mutation":"changeSceneSunIntensity","new_intensity":1.2}                                                                                                                        | 💡️change-scene-sun-intensity      | 💡️dims-scene-sun-to-half                |
      | change-scene-ambient-intensity  | {"mutation":"changeSceneAmbientIntensity","new_intensity":0.25}                                                                                                                   | 🔅️change-scene-ambient-intensity  | 🔅️dims-scene-ambient-to-quarter         |
      | change-scene-shadow-enabled     | {"mutation":"changeSceneShadowEnabled","new_enabled":false}                                                                                                                       | 🌑️change-scene-shadow-enabled     | 🌑️switches-scene-shadows-off            |
      | change-scene-material-roughness | {"mutation":"changeSceneMaterialRoughness","new_roughness":0.25}                                                                                                                  | 🪨️change-scene-material-roughness | ✨️polishes-scene-material-to-quarter    |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the committed render scene exactly
    Given the committed before-snapshot every leaf fixture of this vocabulary shares asset://🧬️schema/🧬️mutations/✏️rename-asset/🧪️tests/🏷️renames-asset-hero-to-lead/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-document asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    When the <id> mutation is applied through apply_shooting_mutation
      """
      {"kind": "<id>", "params": <params>}
      """
    And every step of its own computed inverse is applied through apply_shooting_mutation
    Then the projection equals the base projection again, and the two implementations agree
    Examples:
      | id                              | params                                                                                                                                                                            | dir                               | fixture                               |
      | create-asset                    | {"mutation":"createAsset","asset":{"id":"asset-detail","name":"Detail","url":"/mesh/detail.glb","format":"glb","origin":[0.0,4.0,0.0],"orientation":null,"scale":null},"index":0} | ➕️create-asset                    | ➕️appends-asset-detail                  |
      | delete-asset                    | {"mutation":"deleteAsset","id":"asset-prop"}                                                                                                                                      | 🗑️delete-asset                    | 🗑️removes-trailing-asset-prop           |
      | rename-asset                    | {"mutation":"renameAsset","id":"asset-hero","new_name":"Lead"}                                                                                                                    | ✏️rename-asset                    | 🏷️renames-asset-hero-to-lead            |
      | change-asset-url                | {"mutation":"changeAssetUrl","id":"asset-prop","new_url":"/mesh/prop-v2.glb"}                                                                                                     | 🌐️change-asset-url                | 🌐️points-asset-prop-at-v2-mesh          |
      | reorder-assets                  | {"mutation":"reorderAssets","id":"asset-hero","to_index":1}                                                                                                                       | 🔀️reorder-assets                  | 🔀️moves-asset-hero-behind-asset-prop    |
      | drag-assets                     | {"mutation":"dragAssets","asset_ids":["asset-hero","asset-prop","asset-ghost"],"dx":4.0,"dy":-1.0,"dz":0.5}                                                                       | ↔️drag-assets                     | 🚚️offsets-both-assets-and-skips-a-ghost |
      | rotate-assets                   | {"mutation":"rotateAssets","asset_ids":["asset-hero"],"ax":0.0,"ay":0.0,"az":1.0,"angle":1.5}                                                                                     | 🔄️rotate-assets                   | 🔄️spins-asset-hero-about-z              |
      | scale-assets                    | {"mutation":"scaleAssets","asset_ids":["asset-hero"],"sx":2.0,"sy":2.0,"sz":2.0}                                                                                                  | ↕️scale-assets                    | 📏️doubles-asset-hero-scale              |
      | create-shot                     | {"mutation":"createShot","shot":{"id":"shot-macro","label":"Macro","width":128,"height":128,"format":"png","shape":"rectangle"},"index":null}                                     | 📸️create-shot                     | 📸️appends-shot-macro                    |
      | delete-shot                     | {"mutation":"deleteShot","id":"shot-close"}                                                                                                                                       | 🚮️delete-shot                     | 🚫️removes-trailing-shot-close           |
      | rename-shot                     | {"mutation":"renameShot","id":"shot-close","new_label":"Detail"}                                                                                                                  | 🏷️rename-shot                     | 🔤️relabels-shot-close-to-detail         |
      | change-shot-width               | {"mutation":"changeShotWidth","id":"shot-close","new_width":1024}                                                                                                                 | 📏️change-shot-width               | ↔️widens-shot-close-to-1024             |
      | change-shot-height              | {"mutation":"changeShotHeight","id":"shot-close","new_height":768}                                                                                                                | 📐️change-shot-height              | ↕️heightens-shot-close-to-768           |
      | change-shot-format              | {"mutation":"changeShotFormat","id":"shot-wide","new_format":"svg"}                                                                                                               | 🖼️change-shot-format              | 🎨️switches-shot-wide-to-svg             |
      | change-shot-shape               | {"mutation":"changeShotShape","id":"shot-wide","new_shape":"ellipse"}                                                                                                             | ✂️change-shot-shape               | ⭕️rounds-shot-wide-to-ellipse           |
      | reorder-shots                   | {"mutation":"reorderShots","id":"shot-close","to_index":0}                                                                                                                        | 🔃️reorder-shots                   | ⬆️moves-shot-close-to-front             |
      | replace-shot-camera             | {"mutation":"replaceShotCamera","shot_id":"shot-wide","new_camera":{"position":[3.0,-3.0,2.0],"target":[0.0,0.0,0.5],"zoom":1.5,"fov":40.0}}                                      | 📷️replace-shot-camera             | 📷️rewrites-cam-wide-through-shot-wide   |
      | create-saved-camera             | {"mutation":"createSavedCamera","saved_camera":{"id":"cam-top","label":"Top","camera":{"position":[0.0,0.0,20.0],"target":[0.0,0.0,0.0],"zoom":1.0,"fov":50.0}},"index":null}     | 🎥️create-saved-camera             | 🎥️appends-saved-camera-top              |
      | delete-saved-camera             | {"mutation":"deleteSavedCamera","id":"cam-close"}                                                                                                                                 | 🧹️delete-saved-camera             | 🚫️removes-trailing-cam-close            |
      | rename-saved-camera             | {"mutation":"renameSavedCamera","id":"cam-close","new_label":"Tight"}                                                                                                             | 🪪️rename-saved-camera             | 🔤️relabels-cam-close-to-tight           |
      | replace-saved-camera-view       | {"mutation":"replaceSavedCameraView","id":"cam-close","new_camera":{"position":[1.0,-1.0,0.75],"target":[0.0,0.0,1.0],"zoom":4.0,"fov":20.0}}                                     | 🎞️replace-saved-camera-view       | 📍️repositions-cam-close-view            |
      | reorder-saved-cameras           | {"mutation":"reorderSavedCameras","id":"cam-close","to_index":0}                                                                                                                  | 🔁️reorder-saved-cameras           | 🔁️moves-cam-close-to-front              |
      | set-active-shot                 | {"mutation":"setActiveShot","shot_id":"shot-close"}                                                                                                                               | 🎯️set-active-shot                 | 🎯️activates-shot-close                  |
      | set-active-asset                | {"mutation":"setActiveAsset","asset_id":"asset-prop"}                                                                                                                             | 📌️set-active-asset                | 📌️activates-asset-prop                  |
      | change-scene-sun-enabled        | {"mutation":"changeSceneSunEnabled","new_enabled":false}                                                                                                                          | ☀️change-scene-sun-enabled        | ☀️switches-scene-sun-off                |
      | change-scene-sun-azimuth        | {"mutation":"changeSceneSunAzimuth","new_azimuth":315.0}                                                                                                                          | 🧭️change-scene-sun-azimuth        | 🧭️turns-scene-sun-to-315-degrees        |
      | change-scene-sun-elevation      | {"mutation":"changeSceneSunElevation","new_elevation":60.0}                                                                                                                       | 🌅️change-scene-sun-elevation      | 🌅️raises-scene-sun-to-60-degrees        |
      | change-scene-sun-intensity      | {"mutation":"changeSceneSunIntensity","new_intensity":1.2}                                                                                                                        | 💡️change-scene-sun-intensity      | 💡️dims-scene-sun-to-half                |
      | change-scene-ambient-intensity  | {"mutation":"changeSceneAmbientIntensity","new_intensity":0.25}                                                                                                                   | 🔅️change-scene-ambient-intensity  | 🔅️dims-scene-ambient-to-quarter         |
      | change-scene-shadow-enabled     | {"mutation":"changeSceneShadowEnabled","new_enabled":false}                                                                                                                       | 🌑️change-scene-shadow-enabled     | 🌑️switches-scene-shadows-off            |
      | change-scene-material-roughness | {"mutation":"changeSceneMaterialRoughness","new_roughness":0.25}                                                                                                                  | 🪨️change-scene-material-roughness | ✨️polishes-scene-material-to-quarter    |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real committed shooting DSL artifact
    Given the plugin's own committed DSL artifact asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When it is parsed with parse_dsl and printed back with print_dsl
    Then the printed bytes are identical to the committed bytes and reparsing preserves the projection
      """
      {"kind": "identity-round-trip", "params": {"carrier": "byte-exact"}}
      """
