"""🔒 Observe the native root-view source checkpoint without compiler policy changes."""
import hashlib, json, pathlib, subprocess

root = pathlib.Path(__file__).resolve().parent
repo = root.parents[6]
base = repo / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any"
validation = base / "🧬️schema/⚖️validation"
generated = root / "🗑️generated"
native = [validation / name for name in ("machine.rs", "machine-expression.rs", "machine-operation.rs", "machine-statement.rs", "machine-population.rs", "machine-domain.rs", "machine-rules.rs", "machine-root.rs", "machine-root-tests.rs", "machine-root-fixtures.rs")]
owned = native + [validation / name for name in ("root-read-cases.json", "root-read-cases.schema.json", "root-memo-cases.json", "root-memo-cases.schema.json", "root-read.test.ts")] + [root / name for name in ("native-express-root-read-plan.md", "native-express-root-read-red.md", "native-express-root-read-implementation.md", "native-root-read-fixture.ts", "native-root-read-oracle.py", "native-root-read-freeze.py")]
references = [validation / name for name in ("ast.rs", "program.rs", "program-index.rs", "metadata.rs", "machine-custody-tests.rs", "machine-local-tests.rs", "machine-model-tests.rs", "machine-typeof-tests.rs", "retained.rs", "receive.rs", "overlay.rs", "evaluator.ts", "population.ts", "read-scope.ts", "🧪️vector/source.ifc", "🧪️placement/source.ifc", "🧪️vector-arithmetic/cases.json", "🧪️vector-arithmetic/isolation.json")]

class NativeChecks:
    def parse(self, path):
        result = subprocess.run(["rustfmt", "--edition", "2024", "--config", "skip_children=true", "--emit", "stdout", str(path)], cwd=repo, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE)
        return {"path": str(path), "exit": result.returncode, "stderr": result.stderr.decode("utf-8", "replace")}
    def original_cargo(self):
        argv = ["cargo", "test", "--manifest-path", str(base / "📦️packages/🦀️rust/Cargo.toml"), "--lib", "native_prepared"]
        result = subprocess.run(argv, cwd=repo, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        (generated / "native-root-read-final-cargo.txt").write_bytes(result.stdout)
        return {"argv": argv, "exit": result.returncode, "nativeExecuted": False}

def observation(path):
    data = path.read_bytes()
    length = len(str(path).encode("utf-16-le")) // 2
    if length > 256:
        raise ValueError("source path exceeds256UTF16")
    return {"path": str(path), "bytes": len(data), "sha256": hashlib.sha256(data).hexdigest(), "utf16PathLength": length}

adapter = NativeChecks()
before = [observation(path) for path in owned + references]
syntax = [adapter.parse(path) for path in native]
cargo = adapter.original_cargo()
after = [observation(path) for path in owned + references]
output = {"scope": "Native root-view source plus authored laws; no compile/runtime/receipt proof", "owned": after[:len(owned)], "references": after[len(owned):], "stationary": before == after, "syntax": syntax, "cargo": cargo, "nativeExecuted": False, "publicationEnabled": False}
(generated / "native-root-read-source.json").write_text(json.dumps(output, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
print("[DEBUG] " + json.dumps({"owned": len(owned), "references": len(references), "stationary": before == after, "syntaxExits": [row["exit"] for row in syntax], "cargoExit": cargo["exit"]}))
