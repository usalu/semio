# Browser Output Path Review

Read-only current CLI/runbook audit; no browser/server/tests started. Explicit --entry and --locale are mandatory and validated before server acquisition; no default locale remains. --out is mandatory, but currently only presence is checked.

Concrete gaps sent runtime: arbitrary --out is not checked for containment in an actual ticket generated directory. Arbitrary --tag is joined directly into output and can contain parent traversal, separators, Windows-invalid colon characters or excessive length. Neither CLI nor runJourney API currently preflights256UTF16 absolute output limits before server acquisition/mkdir. Pure validation should cover both callers without relying on the runbook's safe examples.

The current runbook's actual checkout paths are short: authored-de run directory137UTF16, example exported filename171, browser scratch149. Current artifact/stage filenames use canonical short artifact IDs and fixed stage names, never owner IDs containing colon/slash/hash. JSON/Markdown reports and failure screenshot names are likewise safe with a validated tag. Browser scratch is explicitly under the ticket output rather than an external temp location; future generated suffixes still require an explicit available path budget.

On Windows the runbook provides PowerShell SEMIO_LANGUAGE setup, while command arguments remain the same. Git/source paths retain their emoji taxonomy. No concrete owner-ID filename leak was found. These source checks do not execute Chromium or certify all browser-internal profile filenames.
