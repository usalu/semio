"""N1: validates every norm mutation fixture snapshot against its family's committed snapshot JSON Schema, with the
third-party `jsonschema` library as the oracle. Prints per-family counts and the first error of each failing file."""
import glob, json, os, sys
import jsonschema
ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
total_bad = 0
for family in sorted(os.listdir(ROOT)):
    subset = f"{ROOT}/{family}/🏅️standards/🔖️1/🪆️subsets/✳️any"
    schema_path = f"{subset}/🧬️schema/📸️snapshot/🔣️.json"
    if not os.path.isfile(schema_path):
        continue
    schema = json.load(open(schema_path, encoding="utf-8"))
    validator = jsonschema.validators.validator_for(schema)(schema)
    files = sorted(glob.glob(f"{subset}/🧫️fixtures/🧬️mutations/*/*/📸️snapshot/*/🔣️.json"))
    bad = []
    for path in files:
        errors = sorted(validator.iter_errors(json.load(open(path, encoding="utf-8"))), key=lambda e: list(e.path))
        if errors:
            bad.append((path, errors[0]))
    total_bad += len(bad)
    print(f"{family}: {len(files) - len(bad)}/{len(files)} snapshot fixtures valid")
    for path, error in bad[: int(sys.argv[1]) if len(sys.argv) > 1 else 2]:
        print(f"   {path.split('/🧬️mutations/')[1][:90]}: {'/'.join(map(str, error.path))}: {error.message[:140]}")
sys.exit(1 if total_bad else 0)
