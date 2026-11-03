# r2-it: integration test vs summer-strategy clip

## Findings
- Referencing tests: `V/🧪️tests/🐍️integration.py:70-75` (`test_summer_occupant_follows_house_scale` calls `summer-strategy._person(house, 0.0)`) and `V/🧪️tests/🔣️integration.json:419-420` (scene `Beat1_LastZuerstSenken`, plus four more cases at 430-464).
- Current clip (`❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py`): `_person` (line 99) and `class Beat1_LastZuerstSenken` (line 148) both exist. `_person` is a handcrafted cast (`shared-person-fanning` via `staffage_motion`), not a primitive drawing. The fb errors were a transient mid-edit state of the clip; no test or clip change was needed.
- One further error in my first run: `Unknown cast 'cooling-summer-strategy-occupant'` from `_person`, also a transient state (the clip now uses `shared-person-fanning`, present in the manifest).
- Scene reference sweep (`T/🧰️it/scene_refs.py`): every `script`+`scene` pair in architecture test/clip JSON, 317 pairs. All resolve to an existing class. The only two misses are intentional fixtures (`🧪️ready`, `🧪️missing`) in a hierarchy cases file.
- No other test references removed scene classes or helpers.

## Changes
- None to tests or clips.

## Tests (via `T/m9_tests.sh`)
- First batch (integration, cooling clip tests, internal-gains tests, ventilation volume tests, ventilation architecture): 23 tests, 1 error (the transient unknown-cast above).
- Re-run of `test_summer_occupant_follows_house_scale` after the clip moved on: 1 test, OK. Remaining 22 tests of the batch passed.

## Open
- None known.
