
import { execSync } from "child_process";

const repoCli = "@semio-repo/go/go";

function run(command: string) {
  try {
    return execSync(command, { encoding: "utf8", maxBuffer: 10 * 1024 * 1024, stdio: 'pipe' });
  } catch (e: any) {
    // console.warn(`Command failed: ${command}\n${e.message}`);
    return e.stdout || "";
  }
}

interface GoalNode {
  [title: string]: GoalNode;
}

const goalTree: GoalNode = {
  "r26-02": {
    "Running sketchpad": {
      "Running sketchpad Apps": {
        "Running Home App": {},
        "Running Kit App": {},
        "Running Type App": {},
        "Running Design App": {},
        "Running Docs App": {},
      },
      "Updated Docs": {
        "Updated User Docs": {
          "Updated Tutorials": {},
          "Updated Examples": {},
        },
        "Updated Dev Docs": {
          "Updated AGENTS.md": {},
          "Updated README.md": {},
        },
      },
    },
  },
  "r26-03": {
    "Running .NET": {
      "Tested .NET": {},
      "Running Grasshopper": {
        "Pure C# Components": {},
        "Tested Grasshopper Components": {},
      },
    },
  },
  "AI-optimized Repo": {
    "Repo Client": {
      "Repo Binary": {
        "Repo Mechanisms": {
          "Repo Goal Mechanism": {},
          "Repo Ticket Mechanism": {},
          "Repo Draft Mechanism": {},
          "Repo Todo Mechanism": {},
          "Repo Project Mechanism": {},
          "Repo Bundle Mechanism": {},
          "Repo Folder Mechanism": {},
          "Repo File Mechanism": {},
          "Repo Section Mechanism": {},
          "Repo Definition Mechanism": {},
          "Repo Contributor Mechanism": {},
          "Repo Commit Mechanism": {},
          "Repo Policy Mechanism": {},
          "Repo License Mechanism": {},
        },
        "Repo MCP": {
          "Repo MCP Prompts": {},
          "Repo MCP Resources": {},
          "Repo MCP Tools": {},
        },
        "Repo CLI": {
          "Repo CLI Filters": {},
        },
      },
      "Repo VSCode Extension": {},
    },
    "Repo Server": {
      "Repo API": {},
    },
    "Sandboxed Repo": {
      "Zero-Touch Devcontainer": {},
    },
    "Single File Repo": {
      "Consistent Sections": {},
    },
    "Consistent Repo History": {},
  },
};

function slugify(text: string): string {
  return text.toUpperCase().replace(/[^A-Z0-9]+/g, "-").replace(/^-+|-+$/g, "");
}

// Map of Slugified Title -> Goal Info
interface ExistingGoal {
  id: string;
  title: string;
  parentId: string;
}

function getExistingGoals(): Map<string, ExistingGoal> {
  const output = run(`${repoCli} goal list --json`);
  const goals = new Map<string, ExistingGoal>();
  
  const lines = output.trim().split("\n");
  for (const line of lines) {
    if (!line) continue;
    try {
        const json = JSON.parse(line);
        let goal = null;
        if (json.data && json.data.goal) {
            goal = json.data.goal;
        } else if (json.id && json.title) {
             goal = json;
        }

        if (goal) {
            goals.set(slugify(goal.title), {
                id: goal.id,
                title: goal.title,
                parentId: goal.parent || "",
            });
        }
    } catch (e) {
    }
  }
  return goals;
}

const existingGoals = getExistingGoals();

function ensureGoal(title: string, parentPath: string): string {
  const slug = slugify(title);
  const expectedId = parentPath ? `${parentPath}/${slug}` : slug;
  
  const existing = existingGoals.get(slug);

  if (existing) {
    if (existing.id !== expectedId) {
      console.log(`Moving goal '${title}' from '${existing.id}' to '${expectedId}'...`);
      let cmd = `${repoCli} goal change "${existing.id}"`;
      if (parentPath) {
          cmd += ` --parent "${parentPath}"`;
      } else {
         cmd += ` --parent ""`;
      }
      const out = run(cmd);
      // console.log(out);
      existing.id = expectedId;
      existing.parentId = parentPath;
    }
    return expectedId;
  } else {
    console.log(`Creating goal '${title}' at '${expectedId}'...`);
    let cmd = `${repoCli} goal open "${title}" "Goal: ${title}" "gemini-3-pro" "copilot-chat" --no-github`;
    if (parentPath) {
        cmd += ` --parent "${parentPath}"`;
    }
    const out = run(cmd);
    // console.log(out);
    existingGoals.set(slug, { id: expectedId, title, parentId: parentPath });
    return expectedId;
  }
}

function processNode(node: GoalNode, parentPath: string) {
  for (const [title, children] of Object.entries(node)) {
    const currentId = ensureGoal(title, parentPath);
    processNode(children, currentId);
  }
}

console.log("Starting goal reorganization...");
processNode(goalTree, "");
console.log("Goal reorganization complete.");
