# Staffage Website Startup Audit

The existing site is at 🎓️teaching/🏛️architecture/🧍️staffage/🌐️site and is a static application with a development target on port 6071. The repository targets are @teaching/staffage-site:dev, :test, and :build. No built site was present and port 6071 did not respond.

The repository MCP goals resource returned an empty listing; the CLI goal list showed Running Products, under which the ticket was opened. The MCP ticket appeared in the tickets resource but did not create a local ticket directory, so this directory records the audit.

On 2026-11-05 the dev and test targets remained in repository capacity admission without starting: each requested 3584 MB above the 2035 MB host floor, while approximately 500–1600 MB was available above that floor. A single-worker run had the same admission requirement. No application behavior or tests have yet been verified. Resume the site development and test targets when sufficient memory is available, then verify browser flows and build.

## Recheck

The site still did not respond on port 6071. A fresh single-worker development run again waited at admission for 3584 MB above the floor; observed headroom during the run was 7–307 MB. The site test target contains host-document checks, while @teaching/staffage-react has a separate test target covering archive interactions. Full acceptance requires those tests, the site build, and live browser verification after admission succeeds.

A third startup attempt remained in capacity admission; the site still did not respond. Capacity reported 833 MB free above its floor against the 3584 MB request. Startup and verification require the host memory situation to change.
