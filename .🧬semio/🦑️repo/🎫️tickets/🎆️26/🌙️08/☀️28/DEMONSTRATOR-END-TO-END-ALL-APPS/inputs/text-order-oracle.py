import json, sqlite3, sys
from pathlib import Path
rows=json.loads(Path(sys.argv[1]).read_text())['cases']
connection=sqlite3.connect(':memory:')
for row in rows:
    if 'repeat' in row:
        prefix=row['prefix']*row['repeat']
        row={**row,'left':prefix+row['leftSuffix'],'right':prefix+row['rightSuffix']}
    actual=connection.execute('SELECT CASE WHEN ? COLLATE BINARY < ? COLLATE BINARY THEN \'less\' WHEN ? COLLATE BINARY > ? COLLATE BINARY THEN \'greater\' ELSE \'equal\' END',(row['left'],row['right'],row['left'],row['right'])).fetchone()[0]
    assert actual==row['expected'], (row['expected'],actual)
print('SQLite BINARY original text order oracle:',len(rows),'cases passed')
