"""🧾 Compare catalog `volumes` with the ticket source for the r5 object assets."""
import json, pathlib, sys
C = pathlib.Path('/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects')
for s in sys.argv[1:] or ['patio-heater', 'retractable-bollard', 'stepladder', 'street-lamp', 'tealight']:
    c = json.loads((C / s / '🔣️.json').read_text())['asset']['volumes']
    v = json.loads(pathlib.Path(f'volumes/objects/{s}.json').read_text())['volumes']
    print(s, 'equal' if c == v else 'DIFFER')
