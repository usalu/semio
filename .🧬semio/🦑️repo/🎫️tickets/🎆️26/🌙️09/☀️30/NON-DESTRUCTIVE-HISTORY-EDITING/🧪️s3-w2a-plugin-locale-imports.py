"""🧪️ S3-W2A: one mechanical, idempotent pass over the plugin crate's test and fixture sites that still named the os
kernel's former `LocalizedLabel`/`Locale`/`Terminology` re-export (`protocol::…`, `crate::…`) — the kernel now imports them
privately from `semio_framework_ui_locale` (auto-commit 202c4b7b5b1) — plus the registry's new home of
`artifact_schema_descriptor_registered`. Only the files rustc named are touched; running it twice changes nothing."""

import pathlib
import re

PLUGIN = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin")
FILES = [
    "🧪️tests/⚖️declared-verb-verdicts/🦀️.rs",
    "🛂️describe/🧪️tests/🔬️example-assets/🦀️.rs",
    "🏗️builder/🧪️tests/🪪️artifact-admission/🦀️.rs",
    "🏗️builder/🧪️tests/🔬️plugin-builder-dependency/🦀️.rs",
    "🧪️tests/🛰️declaration-channels-unit/🦀️.rs",
    "🧫️fixtures/🛰️declaration-channels/1standard/🌐️any/🧬️mutations/📝️set-value/🦀️.rs",
    "🧫️fixtures/🛰️declaration-channels/1standard/🔒️strict/🧬️mutations/📝️set-value/🦀️.rs",
    "🧫️fixtures/🛰️declaration-channels/2standard/🌐️any/🧬️mutations/📝️set-value/🦀️.rs",
    "⚛️reactor/💼️jobs/🧬️mutation-plan/🧫️fixtures/🧬️job-test-mutations/🧬️mutations/➕️add-value/🦀️.rs",
    "⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️tests/🧬️job-test-mutations-mutations-add-value-unit/🦀️.rs",
    "⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️tests/🔬️unit/🦀️.rs",
    "🧫️fixtures/📢️publication-fixtures/👥️presence/🧬️mutations/📝️change-publication/🦀️.rs",
    "🧫️fixtures/📢️publication-fixtures/🫧️transient/🧬️mutations/📝️change-publication/🦀️.rs",
    "🧫️fixtures/🖥️test-app-mutations/🎚️config/🧬️mutations/📝️change-test-config/🦀️.rs",
    "🧫️fixtures/🖥️test-app-mutations/🧬️document/🧬️mutations/📝️set-test-count/🦀️.rs",
    "🧫️fixtures/🖥️test-app-mutations/🧬️document/🧬️mutations/🏷️set-label/🦀️.rs",
    "🧫️fixtures/🖥️test-app-mutations/🧬️document/🧬️mutations/🧒️set-slot-children/🦀️.rs",
    "🧫️fixtures/📡️contributed-mutation-wire/🧬️mutations/➕️add-value/🦀️.rs",
    "🧪️tests/📡️contributed-mutation-wire-unit/🦀️.rs",
    "🧫️fixtures/🧬️mutation-fixtures/🎲️dummy/🧬️mutations/📝️set-dummy-count/🦀️.rs",
    "🧫️fixtures/🧬️mutation-fixtures/🪟️surface/🧬️mutations/📝️set-surface-count/🦀️.rs",
    "🧫️fixtures/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📝️set-transaction/🦀️.rs",
    "🧫️fixtures/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📣️set-transaction/🦀️.rs",
    "🧫️fixtures/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/⏩️set-transaction/🦀️.rs",
    "🧪️tests/🧪️composed-child-history/🦀️.rs",
    "🏗️builder/🧫️fixtures/🔗️dependency-contribution/🧬️mutations/➕️add-value/🦀️.rs",
    "🏗️builder/🧪️tests/🔗️dependency-contribution-unit/🦀️.rs",
    "🧪️tests/🔬️plugin-runtime-contributed-mutation-wire/🦀️.rs",
    "🧪️tests/🧩️composition/🦀️.rs",
]
NAMES = re.compile(r"(?<![\w:])(?:::)?(?:protocol|crate)::(LocalizedLabel|Locale|Terminology)\b")
LIST_IMPORT = re.compile(r"^use protocol::\{([^}]*)\bLocalizedLabel,\s*([^}]*)\};$", re.M)


def main():
    changed = 0
    for relative in FILES:
        path = PLUGIN / relative
        text = path.read_text(encoding="utf-8")
        new = LIST_IMPORT.sub(lambda match: f"use protocol::{{{match.group(1)}{match.group(2)}}};\nuse semio_framework_ui_locale::LocalizedLabel;", text)
        new = NAMES.sub(lambda match: f"semio_framework_ui_locale::{match.group(1)}", new)
        new = new.replace("semio_framework_schema::artifact_schema_descriptor_registered", "semio_framework_schema_registry::artifact_schema_descriptor_registered")
        if new != text:
            path.write_text(new, encoding="utf-8")
            changed += 1
    print(f"{changed} of {len(FILES)} files changed")


if __name__ == "__main__":
    main()
