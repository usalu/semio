"""Writes the strict tsconfig of the setup-g batch for a tree root (the checkout after landing, or the scratch copy). usage: python3 mktsconfig.py <root> <out.json> a|b|runtime"""
import json, sys
root, out, group = sys.argv[1].rstrip("/"), sys.argv[2], sys.argv[3]
L = "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
R = "/🧰️framework/🔨️modules/⚙️runtime/"
sets = {
 "a": [L + "🏗️build/🧰️tool-setup/🟦️.ts", L + "🏗️build/🧰️tool-setup/🧪️tests/🪜️steps/🟦️.ts", L + "🏗️build/🩺️preflight/🟦️.ts", L + "🏗️build/🩺️preflight/🧪️tests/🧰️toolchain/🟦️.ts"],
 "b": [L + "🏗️build/🧰️tool-setup/⌨️command/🟦️.ts", L + "🏃️process/🦀️cargo/🧰️native-helpers/🟦️.ts"],
 "runtime": [R + "🧲️native/🧰️toolchain/🟦️.ts", R + "🧲️native/🧰️toolchain/🧪️tests/🟦️.ts", R + "🟢️node/🧲️native/🏗️addon/🛠️prepare/🟦️.ts", R + "🟢️node/🧲️native/🏗️addon/🛠️prepare/📜️script.ts", R + "🟢️node/🧲️native/🏗️addon/🛠️prepare/🧪️tests/🟦️.ts"],
}
json.dump({"compilerOptions": {"strict": True, "noEmit": True, "target": "ES2022", "module": "ESNext", "moduleResolution": "Bundler", "allowImportingTsExtensions": True, "resolveJsonModule": True, "skipLibCheck": True, "types": ["node", "bun"], "lib": ["ES2022", "DOM"], "noUncheckedIndexedAccess": False}, "files": [root + f for f in sets[group]]}, open(out, "w", encoding="utf8"), ensure_ascii=False, indent=1)
