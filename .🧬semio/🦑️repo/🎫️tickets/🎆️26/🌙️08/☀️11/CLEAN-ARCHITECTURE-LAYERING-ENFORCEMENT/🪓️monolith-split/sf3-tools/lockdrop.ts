// 🔒️ removes dangling package blocks (path packages whose dependency closure the framework lock does not carry; `update --workspace --offline` adds them).
import {lockDrop} from "../dm2-tools/dd.ts";
for (const n of ["semio-framework-io-sqlite-snapshot-testing-backing", "semio-framework-test-original-snapshot-owners", "semio-framework-test-typed-input-law"]) lockDrop("🧰️framework/Cargo.lock", n);
