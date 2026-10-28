# Course Assignment Exploration

## Existing Contract

The architecture course manifest is 🎓️teaching/🏛️architecture/📚️course/🔣️.json. It references the framework Course definition at 🧰️framework/🛍️products/📚️course/🧬️schema/🔣️.json. Course and Topic both reject additional properties. Course currently supports quizzes, topics, badges, introduction and unlocks; Topic groups quiz IDs only. No assignment or printable attachment field exists. Do not inject an unvalidated manifest field or pretend the worksheet is a scored quiz.

The browser material module 🎓️teaching/🏛️architecture/📚️course/📚️course/🟦️.ts explicitly imports course and quizzes. Its test is ../🧪️tests/🧪️course/🟦️.ts. The Vite builder imports semioReferencedAssetsVitePlugin(repoRoot), so a statically imported new URL asset can use the established asset pipeline. Stack courseFiles currently watches only the course and listed quizzes; an assignment source would require explicit graph/watch support if dynamically consumed. No descendant AGENTS.md was found under teaching during exploration.

## Schema-First Integration Recommendation

Place authored print source under the parametric topic in a dedicated assignment taxonomy, alongside its own JSON schema, language-agnostic fixture and tests. For immediate print delivery, keep it a standalone authored artifact; this avoids expanding the shared proctor wire contract merely to publish a worksheet. If the website must list assignments, first add a render-independent Assignment definition and typed Course assignments / Topic assignment references, project the schema to TypeScript and Rust, add cross-language schema conformance fixtures, then add a localized download link in the course UI and verify the deployed asset response. Audit both source and package schema projections before changing the shared contract; wire fingerprint tests pin structural changes.

## Interpretation and Pedagogy Audit

Interpret the three box panels as three principle-specific comparisons: each shows one good L/B/H parametrization and one deliberately bad case for uniqueness, consistency or minimality. Keep L/B/H defined once with dimension arrows. Uniqueness must expose ambiguous or missing geometric determination; consistency must expose incompatible constraints, rather than merely too many controls; minimality must expose a derivable redundant quantity while preserving a consistent geometry. These principles overlap, so explain which counterexample is isolated and why.

Use two independent tables with eight object rows each, alternating spherical-ended capsules and flat-ended cylindrical capsules. Keep specimens unnamed so choosing a useful parameter vocabulary remains the task. Exactly one Parameter name column plus five empty value columns means six learner-entry columns; the object image may occupy a separate unlabelled reference column. Do not fill in suggested parameter labels or values. Make shape differences visible in monochrome print and avoid perspective ambiguity about a flat versus hemispherical end.

Each K8 graph has exactly 28 unordered edges. If learners annotate every pairwise transformation, provide a dedicated row or equivalent substantial writing space for each edge; labels A–H are reference keys, not shape names. Two graphs imply two separate sets of 28 transitions. A dense complete graph alone cannot provide readable writing space. State whether direction matters: one undirected edge describes the parameter differences between a pair; bidirectional procedures would require 56 directed changes and must not be silently substituted.

A suitable learning sequence is diagnose box parametrizations, discover consistent parameters for each family, then explain pairwise changes. The worksheet should make the generalization goal explicit while withholding the parameter answer. Include a visible unit convention, student/date area, monochrome legible graphics and adequate handwriting height. A concise instructor explanation can live in a separate audit rather than contaminate the blank student tables.

## Verification Scope

Read-only exploration through repo tools exec --light node only. No implementation files changed and no tests executed by this exploration agent.
