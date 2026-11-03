import sys
sys.path.insert(0, sys.argv[1])
from edit_py_helper import edit
edit(sys.argv[2], [('"environment": "office-neutral", "transport": ["parcels"] }', '"environment": "office-neutral", "transport": ["parcels"], "transportAgeS": 3 }')])
