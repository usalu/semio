import { withRepoMcp, ticketDir } from "./repo-mcp.mjs";
import { join } from "path";

const dup = ticketDir("APPS-RUNNING-END-TO-END-AFTER-RESTRUCTURE");
const result = await withRepoMcp(({ tool }) =>
  tool("ticket_close", {
    path: "26/08/06/APPS-RUNNING-END-TO-END-AFTER-RESTRUCTURE",
    summary:
      "Duplicate of open ticket 26/08/06/APPS-RUNNING-END-TO-END covering the same apps E2E work. Continuing on the existing ticket.",
    files: [join(dup, "ticket.json").replace("ticket.json", "🎫️ticket.json")],
  }),
);
console.log(JSON.stringify(result, null, 2));
