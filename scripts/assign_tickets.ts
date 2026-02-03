
import { execSync } from "child_process";

const repoCli = "@semio-repo/go/go";

function run(command: string) {
  try {
    return execSync(command, { encoding: "utf8", maxBuffer: 10 * 1024 * 1024 });
  } catch (e: any) {
    console.error(`Command failed: ${command}\n${e.message}`);
    return "";
  }
}

interface Ticket {
  id: string; // YYYY/MM/DD/SLUG
  slug: string;
  title: string;
  status: string;
}

interface Goal {
  id: string; // SLUG
  title: string;
}

function getTickets(): Ticket[] {
  const json = run(`${repoCli} ticket list --json`);
  if (!json) return [];
  
  const tickets: Ticket[] = [];
  const lines = json.trim().split("\n");
  for (const line of lines) {
    if (!line) continue;
    try {
      const obj = JSON.parse(line);
      // Expected format: {"kind":"result", "data": { "ticket": { ... } }}
      if (obj.kind === "result" && obj.data && obj.data.ticket) {
        const t = obj.data.ticket;
        tickets.push({
            id: t.id || `${t.year}/${String(t.month).padStart(2,'0')}/${String(t.day).padStart(2,'0')}/${t.slug}`,
            slug: t.slug,
            title: t.title,
            status: t.status
        });
      }
    } catch (e) {}
  }
  return tickets;
}

function getGoals(): Goal[] {
  const json = run(`${repoCli} goal list --json`);
  if (!json) return [];
  
  const goals: Goal[] = [];
  const lines = json.trim().split("\n");
  for (const line of lines) {
    if (!line) continue;
    try {
      const obj = JSON.parse(line);
      if (obj.kind === "result" && obj.data && obj.data.goal) {
          // Goal ID is likely the slug, but if not present, we must look for it.
          // The JSON sample showed keys: "goal": {"title"..., "parent"...}
          // The ID is separate? 
          // Let's debug this.
          // console.log("Available keys in data:", Object.keys(obj.data));
          
          let id = obj.data.id || obj.data.slug;
          
          // Fallback: slugify title if id is missing (dangerous but trying)
          // Actually, let's verify if `id` exists in `data`.
          
          if (!id && obj.data.goal.title) {
               // Temporary fix: assume slug from title.
               // Replace non-alphanumeric (except dashes) with dashes.
               id = obj.data.goal.title.toUpperCase().replace(/[^A-Z0-9]+/g, '-');
               // Remove leading/trailing dashes if any
               id = id.replace(/^-+|-+$/g, '');
          }
          
          const g = obj.data.goal;
          goals.push({
              id: id,
              title: g.title
          });
      }
    } catch (e) {}
  }
  return goals;
}

// Map keywords to Goal IDs
const rules: Array<[RegExp, string]> = [
    [/vscode|extension/i, "REPO-VSCODE-EXTENSION"],
    [/cli|command/i, "REPO-CLI"],
    [/mcp/i, "REPO-MCP"],
    [/server/i, "REPO-SERVER"],
    [/ticket/i, "REPO-TICKET-MECHANISM"],
    [/goal/i, "REPO-GOAL-MECHANISM"],
    [/section/i, "REPO-SECTION-MECHANISM"],
    [/definition/i, "REPO-DEFINITION-MECHANISM"],
    [/file/i, "REPO-FILE-MECHANISM"],
    [/folder/i, "REPO-FOLDER-MECHANISM"],
    [/bundle/i, "REPO-BUNDLE-MECHANISM"],
    [/policy/i, "REPO-POLICY-MECHANISM"],
    [/license/i, "REPO-LICENSE-MECHANISM"],
    [/contributor/i, "REPO-CONTRIBUTOR-MECHANISM"],
    [/commit/i, "REPO-COMMIT-MECHANISM"],
    [/devcontainer/i, "ZERO-TOUCH-DEVCONTAINER"],
    [/api/i, "REPO-API"],
    [/sketchpad/i, "RUNNING-SKETCHPAD"],
    [/design/i, "RUNNING-DESIGN-APP"],
    [/type/i, "RUNNING-TYPE-APP"],
    [/kit|metabolism/i, "RUNNING-KIT-APP"],
    [/home/i, "RUNNING-HOME-APP"],
    [/docs/i, "RUNNING-DOCS-APP"],
    [/net|csharp|c#/i, "RUNNING-NET"],
    [/grasshopper/i, "RUNNING-GRASSHOPPER"],
    [/agent|report/i, "UPDATED-AGENTS-MD"],
    [/example/i, "UPDATED-EXAMPLES"]
];

async function main() {
    console.log("Fetching tickets...");
    const tickets = getTickets();
    console.log(`Found ${tickets.length} tickets.`);

    console.log("Fetching goals...");
    const goals = getGoals();
    console.log(`Found ${goals.length} goals.`);
    
    const validGoalIds = new Set(goals.map(g => g.id));
    // console.log("Valid Goal IDs:", Array.from(validGoalIds));

    let assignedCount = 0;
    
    for (const ticket of tickets) {
        let targetGoalId: string | undefined;

        for (const [regex, goalId] of rules) {
            if (regex.test(ticket.slug) || regex.test(ticket.title)) {
                if (validGoalIds.has(goalId)) {
                    targetGoalId = goalId;
                    break;
                } else {
                    // console.log(`Match found ${goalId} for ${ticket.slug} but goal ID not in list.`);
                }
            }
        }

        if (targetGoalId) {
            console.log(`Assigning ${ticket.slug} -> ${targetGoalId}`);
            run(`${repoCli} ticket change "${ticket.id}" --goal "${targetGoalId}" --no-github`);
            assignedCount++;
        }
    }

    console.log(`Assigned ${assignedCount} tickets.`);
}

main();
