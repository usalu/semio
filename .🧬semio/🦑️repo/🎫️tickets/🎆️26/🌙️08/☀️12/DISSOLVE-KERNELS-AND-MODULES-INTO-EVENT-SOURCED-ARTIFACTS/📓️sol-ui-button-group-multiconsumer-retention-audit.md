# UI Button Group Multi-Consumer Retention Audit

- Component SHA-256: `53a093cbca39a9728da1bd0ba1d3978c9a9f4e7d978056bcbd19b9e6bab9b789`, clean.
- Story SHA-256: `a54adaacd19300fe2bf3406fc1c93aa3393c1f482f254518cadffc02eb0192f3`, clean.
- React index at audit time: `f4415689af8fadf41714bde7b4bc7181169804a7b878ee25411791ec8d5abf59`.

ButtonGroup and ButtonGroupItem are directly rendered by multiple independent active production components: framework Button, framework Canvas, protected renderer UtilityTree, protected renderer ShellHost, and two independent React package engagement renderers. Stories and inline tests do not affect the qualifying count; renderer package-index imports are glue.

Decision: retain ButtonGroup at the framework UI owner. It exceeds the two-independent-production-consumer threshold and is already at the lowest common owner across framework and OS renderer terminals. No source edit follows.
