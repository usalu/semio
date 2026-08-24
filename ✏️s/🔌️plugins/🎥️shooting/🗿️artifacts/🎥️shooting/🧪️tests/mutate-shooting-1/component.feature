@capability-shooting-1-mutate
@no-oracle-shooting-render-scene-mutation-semantics
@comparison-ordered-json-v1
@mutations-shooting-1-any
Feature: Apply every typed SHOOTING mutation to the vocabulary's own committed render scene
  `s.shooting.shooting` is a semio-NATIVE artifact — the `shooting.shooting.dsl` grammar, with its typed
  table columns and its `deg`-suffixed angle literals, is defined by this repository alone — so this case
  carries a recorded no-oracle decision (`shooting-render-scene-mutation-semantics`, in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`) rather than a registered reference
  library. That decision does not rest on absence: it surveys glTF 2.0, USD and Collada by name, which
  really do model an asset list, a transform, a camera and a punctual light — and rejects them on the one
  structural point that matters here, that none of them models a SHOT. A shot is a render request
  (width, height, output format, mask shape, background, saved-camera reference) and ELEVEN of the
  thirty-one kinds address it, so a glTF reader asked to judge `change-shot-format` would have nothing to
  read and would report agreement about a field it does not carry. ⚠️ Consequence, stated plainly: the
  runner dispatches NO oracle role for a recorded no-oracle case, so every scenario below carries its
  evidence in the SUBJECT role or carries none at all. A handler that merely applied the mutation and
  returned would report a pass having checked nothing, which is why each one asserts its law through the
  shared `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law/🦀️component.rs` module before it returns.

  📄️ The base document is real, committed, and is not this case's invention in any part. All thirty-one
  of this vocabulary's per-kind leaf fixtures under
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<kind>/🧪️tests/` commit the SAME
  before-snapshot, byte for byte — SHA-1 `6441b72754e5c649b2b07a2f2b244313467f85a0`, verified across all
  thirty-one copies — and this case reads that one document where the domain already keeps it, at
  `asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-asset/🧪️tests/renames-asset-hero-to-lead/📸️snapshot/⬅️before/🔣️component.json`.
  It carries two assets (one with a quaternion orientation and a non-unit scale, one with neither), two
  saved cameras, two shots (one with a background and a camera reference, one with neither), a full scene
  lighting block and both active-selection ids. Every `params` cell below is likewise the committed
  payload of that kind's OWN leaf fixture, transcribed verbatim and not reworded — this is the one subset
  in this wave whose committed fixtures pin POSITIVE branches rather than guards, because
  `ShootingSnapshot` holds its assets, shots, cameras and scene INLINE and mints no content-addressed
  child handle, so its leaf authors could state a real `➡️after` by hand. All thirty-one were checked
  against the shared before-state and every one of them moves it.

  🎞️ The `identity-round-trip` scenario reads a different real committed file on purpose: the plugin's
  own DSL example at
  `asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`.
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
  camelCase — exactly as each committed leaf fixture is.

  ⚖️ The projection is `(schema, assets, savedCameras, scene, shots, activeShotId, activeAssetId)`. The
  composed `emblem` child handle is deliberately NOT projected: it is a content address for an
  `s.stdio.semio.image` child, no kind of this vocabulary addresses it, and the committed base document
  does not carry one.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to the committed render scene and observe it move
    Given the committed before-snapshot every leaf fixture of this vocabulary shares asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-asset/🧪️tests/renames-asset-hero-to-lead/📸️snapshot/⬅️before/🔣️component.json
    When the <id> mutation is applied through apply_shooting_mutation
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the resulting projection differs from the base projection
    Examples:
      | id                              | params                                                                                                                                                                            |
      | create-asset                    | {"mutation":"createAsset","asset":{"id":"asset-detail","name":"Detail","url":"/mesh/detail.glb","format":"glb","origin":[0.0,4.0,0.0],"orientation":null,"scale":null},"index":0} |
      | delete-asset                    | {"mutation":"deleteAsset","id":"asset-prop"}                                                                                                                                      |
      | rename-asset                    | {"mutation":"renameAsset","id":"asset-hero","new_name":"Lead"}                                                                                                                    |
      | change-asset-url                | {"mutation":"changeAssetUrl","id":"asset-prop","new_url":"/mesh/prop-v2.glb"}                                                                                                     |
      | reorder-assets                  | {"mutation":"reorderAssets","id":"asset-hero","to_index":1}                                                                                                                       |
      | drag-assets                     | {"mutation":"dragAssets","asset_ids":["asset-hero","asset-prop","asset-ghost"],"dx":4.0,"dy":-1.0,"dz":0.5}                                                                       |
      | rotate-assets                   | {"mutation":"rotateAssets","asset_ids":["asset-hero"],"ax":0.0,"ay":0.0,"az":1.0,"angle":1.5}                                                                                     |
      | scale-assets                    | {"mutation":"scaleAssets","asset_ids":["asset-hero"],"sx":2.0,"sy":2.0,"sz":2.0}                                                                                                  |
      | create-shot                     | {"mutation":"createShot","shot":{"id":"shot-macro","label":"Macro","width":128,"height":128,"format":"png","shape":"rectangle"},"index":null}                                     |
      | delete-shot                     | {"mutation":"deleteShot","id":"shot-close"}                                                                                                                                       |
      | rename-shot                     | {"mutation":"renameShot","id":"shot-close","new_label":"Detail"}                                                                                                                  |
      | change-shot-width               | {"mutation":"changeShotWidth","id":"shot-close","new_width":1024}                                                                                                                 |
      | change-shot-height              | {"mutation":"changeShotHeight","id":"shot-close","new_height":768}                                                                                                                |
      | change-shot-format              | {"mutation":"changeShotFormat","id":"shot-wide","new_format":"svg"}                                                                                                               |
      | change-shot-shape               | {"mutation":"changeShotShape","id":"shot-wide","new_shape":"ellipse"}                                                                                                             |
      | reorder-shots                   | {"mutation":"reorderShots","id":"shot-close","to_index":0}                                                                                                                        |
      | replace-shot-camera             | {"mutation":"replaceShotCamera","shot_id":"shot-wide","new_camera":{"position":[3.0,-3.0,2.0],"target":[0.0,0.0,0.5],"zoom":1.5,"fov":40.0}}                                      |
      | create-saved-camera             | {"mutation":"createSavedCamera","saved_camera":{"id":"cam-top","label":"Top","camera":{"position":[0.0,0.0,20.0],"target":[0.0,0.0,0.0],"zoom":1.0,"fov":50.0}},"index":null}     |
      | delete-saved-camera             | {"mutation":"deleteSavedCamera","id":"cam-close"}                                                                                                                                 |
      | rename-saved-camera             | {"mutation":"renameSavedCamera","id":"cam-close","new_label":"Tight"}                                                                                                             |
      | replace-saved-camera-view       | {"mutation":"replaceSavedCameraView","id":"cam-close","new_camera":{"position":[1.0,-1.0,0.75],"target":[0.0,0.0,1.0],"zoom":4.0,"fov":20.0}}                                     |
      | reorder-saved-cameras           | {"mutation":"reorderSavedCameras","id":"cam-close","to_index":0}                                                                                                                  |
      | set-active-shot                 | {"mutation":"setActiveShot","shot_id":"shot-close"}                                                                                                                               |
      | set-active-asset                | {"mutation":"setActiveAsset","asset_id":"asset-prop"}                                                                                                                             |
      | change-scene-sun-enabled        | {"mutation":"changeSceneSunEnabled","new_enabled":false}                                                                                                                          |
      | change-scene-sun-azimuth        | {"mutation":"changeSceneSunAzimuth","new_azimuth":315.0}                                                                                                                          |
      | change-scene-sun-elevation      | {"mutation":"changeSceneSunElevation","new_elevation":60.0}                                                                                                                       |
      | change-scene-sun-intensity      | {"mutation":"changeSceneSunIntensity","new_intensity":1.2}                                                                                                                        |
      | change-scene-ambient-intensity  | {"mutation":"changeSceneAmbientIntensity","new_intensity":0.25}                                                                                                                   |
      | change-scene-shadow-enabled     | {"mutation":"changeSceneShadowEnabled","new_enabled":false}                                                                                                                       |
      | change-scene-material-roughness | {"mutation":"changeSceneMaterialRoughness","new_roughness":0.25}                                                                                                                  |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed render scene exactly
    Given the committed before-snapshot every leaf fixture of this vocabulary shares asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-asset/🧪️tests/renames-asset-hero-to-lead/📸️snapshot/⬅️before/🔣️component.json
    When the <id> mutation is applied through apply_shooting_mutation
      """
      {"kind": "<id>", "params": <params>}
      """
    And every step of its own computed inverse is applied through apply_shooting_mutation
    Then the projection equals the base projection again
    Examples:
      | id                              | params                                                                                                                                                                            |
      | create-asset                    | {"mutation":"createAsset","asset":{"id":"asset-detail","name":"Detail","url":"/mesh/detail.glb","format":"glb","origin":[0.0,4.0,0.0],"orientation":null,"scale":null},"index":0} |
      | delete-asset                    | {"mutation":"deleteAsset","id":"asset-prop"}                                                                                                                                      |
      | rename-asset                    | {"mutation":"renameAsset","id":"asset-hero","new_name":"Lead"}                                                                                                                    |
      | change-asset-url                | {"mutation":"changeAssetUrl","id":"asset-prop","new_url":"/mesh/prop-v2.glb"}                                                                                                     |
      | reorder-assets                  | {"mutation":"reorderAssets","id":"asset-hero","to_index":1}                                                                                                                       |
      | drag-assets                     | {"mutation":"dragAssets","asset_ids":["asset-hero","asset-prop","asset-ghost"],"dx":4.0,"dy":-1.0,"dz":0.5}                                                                       |
      | rotate-assets                   | {"mutation":"rotateAssets","asset_ids":["asset-hero"],"ax":0.0,"ay":0.0,"az":1.0,"angle":1.5}                                                                                     |
      | scale-assets                    | {"mutation":"scaleAssets","asset_ids":["asset-hero"],"sx":2.0,"sy":2.0,"sz":2.0}                                                                                                  |
      | create-shot                     | {"mutation":"createShot","shot":{"id":"shot-macro","label":"Macro","width":128,"height":128,"format":"png","shape":"rectangle"},"index":null}                                     |
      | delete-shot                     | {"mutation":"deleteShot","id":"shot-close"}                                                                                                                                       |
      | rename-shot                     | {"mutation":"renameShot","id":"shot-close","new_label":"Detail"}                                                                                                                  |
      | change-shot-width               | {"mutation":"changeShotWidth","id":"shot-close","new_width":1024}                                                                                                                 |
      | change-shot-height              | {"mutation":"changeShotHeight","id":"shot-close","new_height":768}                                                                                                                |
      | change-shot-format              | {"mutation":"changeShotFormat","id":"shot-wide","new_format":"svg"}                                                                                                               |
      | change-shot-shape               | {"mutation":"changeShotShape","id":"shot-wide","new_shape":"ellipse"}                                                                                                             |
      | reorder-shots                   | {"mutation":"reorderShots","id":"shot-close","to_index":0}                                                                                                                        |
      | replace-shot-camera             | {"mutation":"replaceShotCamera","shot_id":"shot-wide","new_camera":{"position":[3.0,-3.0,2.0],"target":[0.0,0.0,0.5],"zoom":1.5,"fov":40.0}}                                      |
      | create-saved-camera             | {"mutation":"createSavedCamera","saved_camera":{"id":"cam-top","label":"Top","camera":{"position":[0.0,0.0,20.0],"target":[0.0,0.0,0.0],"zoom":1.0,"fov":50.0}},"index":null}     |
      | delete-saved-camera             | {"mutation":"deleteSavedCamera","id":"cam-close"}                                                                                                                                 |
      | rename-saved-camera             | {"mutation":"renameSavedCamera","id":"cam-close","new_label":"Tight"}                                                                                                             |
      | replace-saved-camera-view       | {"mutation":"replaceSavedCameraView","id":"cam-close","new_camera":{"position":[1.0,-1.0,0.75],"target":[0.0,0.0,1.0],"zoom":4.0,"fov":20.0}}                                     |
      | reorder-saved-cameras           | {"mutation":"reorderSavedCameras","id":"cam-close","to_index":0}                                                                                                                  |
      | set-active-shot                 | {"mutation":"setActiveShot","shot_id":"shot-close"}                                                                                                                               |
      | set-active-asset                | {"mutation":"setActiveAsset","asset_id":"asset-prop"}                                                                                                                             |
      | change-scene-sun-enabled        | {"mutation":"changeSceneSunEnabled","new_enabled":false}                                                                                                                          |
      | change-scene-sun-azimuth        | {"mutation":"changeSceneSunAzimuth","new_azimuth":315.0}                                                                                                                          |
      | change-scene-sun-elevation      | {"mutation":"changeSceneSunElevation","new_elevation":60.0}                                                                                                                       |
      | change-scene-sun-intensity      | {"mutation":"changeSceneSunIntensity","new_intensity":1.2}                                                                                                                        |
      | change-scene-ambient-intensity  | {"mutation":"changeSceneAmbientIntensity","new_intensity":0.25}                                                                                                                   |
      | change-scene-shadow-enabled     | {"mutation":"changeSceneShadowEnabled","new_enabled":false}                                                                                                                       |
      | change-scene-material-roughness | {"mutation":"changeSceneMaterialRoughness","new_roughness":0.25}                                                                                                                  |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real committed shooting DSL artifact
    Given the plugin's own committed DSL artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When it is parsed with parse_dsl and printed back with print_dsl
    Then the printed bytes are identical to the committed bytes and reparsing preserves the projection
      """
      {"kind": "identity-round-trip", "params": {"carrier": "byte-exact"}}
      """
