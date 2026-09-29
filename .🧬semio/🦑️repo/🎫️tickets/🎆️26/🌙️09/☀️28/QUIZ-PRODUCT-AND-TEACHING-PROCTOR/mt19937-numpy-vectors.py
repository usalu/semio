"""🎲️ Third-party MT19937 vectors for the quiz core tests: numpy's legacy `init_genrand` seeding + raw tempered outputs."""
import json
import numpy as np

vectors = {}
for seed in [0, 1, 42, 5489, 2166136261, 4294967295]:
    generator = np.random.MT19937()
    generator._legacy_seeding(seed)
    raw = generator.random_raw(1000)
    vectors[str(seed)] = {"first": [int(value) for value in raw[:5]], "thousandth": int(raw[999])}
print(json.dumps({"numpy": np.__version__, "vectors": vectors}, indent=2))
