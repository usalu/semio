File: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/📦️packages/🦀️rust/Cargo.toml
Change dev-dependency semio-framework-os-kernel features ["mutation-testing"] -> ["mutation-testing", "protocol-laws"]
Reason: job-test-mutations fixture calls semio_framework_os_kernel::os_spr::protocol_laws (gated by spr feature protocol-laws; kernel exposes that feature). E0433 in testport-p2-b2.txt.
