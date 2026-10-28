from pathlib import Path
import json
import sqlite3
import jsonschema

root = Path(__file__).resolve().parents[7]
source = root / '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🗂️registry'
law = json.loads((source / '🧪️tests/🧫️fixtures/🔣️.json').read_text())
schema = json.loads((source / '🧬️schema/🔣️.json').read_text())
jsonschema.Draft202012Validator(schema).validate(law)
for case in law['cases']:
    database = sqlite3.connect(':memory:')
    database.execute('CREATE TABLE original_address(address TEXT PRIMARY KEY, value INTEGER NOT NULL)')
    database.executemany('INSERT INTO original_address VALUES (?, ?)', [(key, 1) for key in case['before']])
    database.commit()
    database.execute('BEGIN')
    database.execute('INSERT INTO original_address VALUES (?, ?)', (law['key'], law['value']))
    assert [row[0] for row in database.execute('SELECT address FROM original_address ORDER BY address')] == case['after']
    database.rollback()
    assert [row[0] for row in database.execute('SELECT address FROM original_address ORDER BY address')] == case['before']
    database.close()
print('[DEBUG] Window registry schema and independent SQLite commit/rollback oracle passed for all cases.')
