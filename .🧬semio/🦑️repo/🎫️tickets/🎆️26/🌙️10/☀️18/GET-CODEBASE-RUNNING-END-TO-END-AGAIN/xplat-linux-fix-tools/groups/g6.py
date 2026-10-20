NAME = 'g6'
TITLE = 'hermetic spawn strategies name only strategies every Bazel host registers: sandboxed resolves to the best sandbox the host has'
REQUIRES = []
EDITS = [
    ('🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/🔒️hermetic/🔣️.json', '    "darwin": [\n      "remote",\n      "darwin-sandbox",\n      "isolated-root"\n    ],\n    "linux": [\n      "remote",\n      "linux-sandbox",\n      "processwrapper-sandbox",\n      "isolated-root"\n    ],', '    "darwin": [\n      "remote",\n      "sandboxed",\n      "isolated-root"\n    ],\n    "linux": [\n      "remote",\n      "sandboxed",\n      "isolated-root"\n    ],'),
    ('🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/🧬️schema/🔒️hermetic-classes.json', '          "remote",\n          "linux-sandbox",\n          "darwin-sandbox",\n          "processwrapper-sandbox",\n          "isolated-root"', '          "remote",\n          "sandboxed",\n          "isolated-root"'),
    ('🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/🔒️hermetic/🧪️tests/🧫️fixtures/🔣️.json', '    {"platform": "darwin", "expected": ["remote", "darwin-sandbox", "local"]},\n    {"platform": "linux", "expected": ["remote", "linux-sandbox", "processwrapper-sandbox", "local"]},', '    {"platform": "darwin", "expected": ["remote", "sandboxed", "local"]},\n    {"platform": "linux", "expected": ["remote", "sandboxed", "local"]},'),
    ('🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🧪️tests/🧫️fixtures/🔣️.json', '"--strategy=RepoHermetic=darwin-sandbox,local"', '"--strategy=RepoHermetic=sandboxed,local"', 1),
    ('🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🧪️tests/🧫️fixtures/🔣️.json', '"--strategy=RepoHermetic=remote,darwin-sandbox,local"', '"--strategy=RepoHermetic=remote,sandboxed,local"', 1),
    ('🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🧪️tests/🧫️fixtures/🔣️.json', '"--strategy=RepoHermetic=remote,linux-sandbox,processwrapper-sandbox,local"', '"--strategy=RepoHermetic=remote,sandboxed,local"', 1),
]
NEW = ['🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🧪️tests/📥️strategies/🟦️.ts',
 '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🧪️tests/📥️strategies/🧫️fixtures/🔣️.json']
