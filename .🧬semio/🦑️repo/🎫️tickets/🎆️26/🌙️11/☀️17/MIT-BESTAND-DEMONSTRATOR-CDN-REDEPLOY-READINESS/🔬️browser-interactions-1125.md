# Demonstrator browser interaction verification

The existing release receipt points to `.cache/b/p/m/vite/release/5dc1b077e86de2aa/out`. A local static server hosted those bytes on `cdn.localhost:8766` with production MIME types. The production browser check was enabled to detect requests to development origins.

The workspace and demonstrator resolved to separate installs of Playwright Test 1.64.1. Running the workspace CLI against the demonstrator spec failed at test registration. Resolving the CLI from the demonstrator let Playwright register and execute the spec. The run discovered 36 cases after excluding the 16 pane boot cases.

The first English run found that the initial language-picker assertion expected an absence of ARIA state attributes. The rendered radio buttons correctly carry `aria-checked="false"`. The navbar now uses the same shared language-choice radiogroup. Updated the existing tests to assert radio roles, checked state, language attributes, and the navbar's language-choice group. The English picker and language switch passed after the change, including reload persistence. The English pane roster, both overview languages, and keyboard overview scroll-focus also passed against the existing release.

Browser runs were interrupted to release memory for the Procedural component link. The English live language switch and Aussuchen preview were interrupted while waiting for pane readiness and page navigation respectively. The current release and remaining interactions still require verification.
