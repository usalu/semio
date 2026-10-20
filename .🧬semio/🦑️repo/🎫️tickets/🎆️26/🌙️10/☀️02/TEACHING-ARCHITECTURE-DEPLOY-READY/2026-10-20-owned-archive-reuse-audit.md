# Existing Owned Archive Reuse Audit

Bounded read-only scope: course deploy tests/fixtures, framework process artifacts/archive and ticket 🔎️bundle. No command, test, Source edit or runtime effect.

Existing public interface @semio-tech/process/artifacts/archive resolves physical framework/🔨️modules/🏃️process/📦️artifacts/🗃️archive/🟦️.ts. Export decodeBoundedArchiveV1(input,request,control) validates policy/cancellation/progress and returns retained entries. Its request schema supports only tar.gz and zip. Hardcaps32MiBcompressed,256MiBinflated,4096entries,256MiBfile,64KiBmetadata. Internal tar framing validates checksums, POSIX/PAX/GNUlongpath, duplicate/canonical paths, ordinarytypes, zero padding/footer. It always inflates gzip then retains full archive, so cannot directly inspect a plain multi-gigabyte Docker-save TAR. Do not add a gzipwrapper or silently widen limits merely to reuse this API.

Owned tests archive/🧪️tests/🟦️.ts:18 use independent Bun.Archive for TAR content, fflate for ZIP, plus Ajv schema and SQLite projections. Cancellation and malformed framing cases exist. This is a useful existing oracle/style for schema-first raw-TAR framing extension/factoring if needed, not evidence of Docker-save support.

Course deploy/📦️image/🟦️.ts owns finite TAR runtime-context writing, not decoding. Existing course sealed-publication-and-bundle tests assert bundle planning/YAML/policies and do not parse Docker-save archives. Ticket 🔎️bundle/📜️script.ts is historical Source/compiler AST attribution and is not an image-archive verifier. No suitable current plain Docker-save parser was found in these exact bounded namespaces.

The narrow bundle tag/config fix needs only validated TAR metadata framing, manifest.json and referenced config bytes: bounded streamskip of opaque layers is sufficient, without unpacking arbitrary image layers. Existing framing logic can be factored under the current archive owner, preserving limits and control rather than adding a second general TAR framework. Runtime received exact paths/caps/oracle limitations.
