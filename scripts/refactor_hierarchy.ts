import * as fs from 'fs';
import * as path from 'path';

// Define path constants to match the environment
const WORKSPACE_ROOT = process.cwd();
const GOALS_DIR = path.join(WORKSPACE_ROOT, '.semio-repo/goals');
const TICKETS_DIR = path.join(WORKSPACE_ROOT, '.semio-repo/tickets');
const LEGACY_TICKETS_DIR = path.join(WORKSPACE_ROOT, 'tickets');

const TARGET_HIERARCHY = {
    "R26-02": {
        "Running Sketchpad": {
            "Apps": {
                "Home App": {},
                "Kit App": {},
                "Type App": {},
                "Design App": {},
                "Quality App": {},
                "Docs App": {},
                "Feedback App": {},
            },
            "Updated Docs": {
                "User Docs": {},
                "Dev Docs": {}
            }
        },
        "Updated AGENTS.md": {}
    },
    "R26-03": {
        "Running .NET": {
            "Tested .NET": {},
            "Running Grasshopper": {
                "Pure C# Components": {},
                "Tested Grasshopper Components": {}
            }
        }
    },
    "AI-optimized Repo": {
        "Repo Client": {
            "Repo Binary": {
                "Repo MCP": {},
                "Repo CLI": {}
            },
            "Repo VSCode Extension": {}
        },
        "Repo Server": {
            "Repo API": {}
        }
    }
};

function normalize(s: string) {
    return s.toLowerCase().trim();
}

function escapeRegExp(string: string) {
    return string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

async function run() {
    console.log("STARTING REFRACTOR SCRIPT");
    console.log(`CWD: ${process.cwd()}`);

    // Load tree.json
    console.log("Reading tree.json...");
    const jsonContent = fs.readFileSync('tree.json', 'utf8');

    let parsed: any = null;
    const lines = jsonContent.split('\n');

    // Try to find the result line in a JSON stream
    for (const line of lines) {
        const trimmed = line.trim();
        if (!trimmed) continue;
        try {
            const json = JSON.parse(trimmed);
            if (json.kind === 'result' && json.data) {
                parsed = json;
                break;
            }
        } catch (e) {
            // ignore
        }
    }

    // Fallback: maybe it's just a single JSON object
    if (!parsed) {
        try {
            parsed = JSON.parse(jsonContent);
        } catch (e) {
            console.error("Failed to parse tree.json");
            process.exit(1);
        }
    }

    // Improved parsing logic
    let tree;
    if (parsed.kind === 'result' && parsed.data?.data?.repo) {
        tree = parsed.data.data.repo;
    } else if (parsed.data?.repo) {
        tree = parsed.data.repo;
    } else {
        tree = parsed.data || parsed;
    }

    if (!tree || !tree.goals) {
        console.error("Invalid tree structure. 'goals' property missing.");
        return;
    }

    const goals = tree.goals;
    console.log(`Loaded ${goals.length} goals.`);

    // Build Lookups
    const goalsByTitle = new Map();
    const titleLUT = new Map();
    goals.forEach((g: any) => {
        goalsByTitle.set(g.title, g);
        titleLUT.set(normalize(g.title), g);
    });

    const oldToNewGoalId = new Map<string, string>();

    async function processNode(node: any, parentGoalId: string) {
        for (const title of Object.keys(node)) {
            let goal = goalsByTitle.get(title);
            if (!goal) {
                goal = titleLUT.get(normalize(title));
            }

            if (!goal) {
                console.warn(`Goal "${title}" not found in tree.json. Skipping.`);
                continue;
            }

            const currentId = goal.id;
            const currentSlug = path.basename(currentId);

            const currentPath = path.join(GOALS_DIR, currentId);

            const desiredId = parentGoalId ? `${parentGoalId}/${currentSlug}` : currentSlug;
            const desiredPath = path.join(GOALS_DIR, desiredId);

            oldToNewGoalId.set(currentId, desiredId);

            // Handle Moves
            if (currentId !== desiredId) {
                if (fs.existsSync(currentPath)) {
                    console.log(`Moving "${title}"\n  From: ${currentId}\n  To:   ${desiredId}`);

                    const parentDir = path.dirname(desiredPath);
                    if (!fs.existsSync(parentDir)) {
                        fs.mkdirSync(parentDir, { recursive: true });
                    }

                    if (fs.existsSync(desiredPath)) {
                        console.log(`  Destination exists. Merging content...`);
                        const items = fs.readdirSync(currentPath);
                        for (const item of items) {
                            const src = path.join(currentPath, item);
                            const dst = path.join(desiredPath, item);
                            if (fs.existsSync(dst)) {
                                if (fs.lstatSync(dst).isDirectory()) {
                                    console.warn(`  Item ${item} exists in destination. Skipping.`);
                                }
                            } else {
                                fs.renameSync(src, dst);
                            }
                        }
                        try { fs.rmdirSync(currentPath); } catch (e) { }
                    } else {
                        fs.renameSync(currentPath, desiredPath);
                    }

                    const goalJsonPath = path.join(desiredPath, 'goal.json');
                    if (fs.existsSync(goalJsonPath)) {
                        try {
                            // Optional: Could update parent pointers here
                        } catch (e) { }
                    }

                }
            }

            await processNode(node[title], desiredId);
        }
    }

    console.log("Refactoring Goals...");
    await processNode(TARGET_HIERARCHY, "");

    console.log(`Updating Ticket Goal References (${oldToNewGoalId.size} potential renames)...`);

    if (tree.tickets) {
        for (const t of tree.tickets) {
            if (!t.id.startsWith('@semio-repo/ticket/')) continue;

            const m = t.id.match(/@semio-repo\/ticket\/(\d{4})\/(\d{1,2})\/(\d{1,2})\/(.+)/);
            if (!m) continue;

            const [_, y, m2, d, s] = m;

            let dir = path.join(TICKETS_DIR, y, m2, d, s);
            if (!fs.existsSync(dir)) {
                dir = path.join(LEGACY_TICKETS_DIR, y, m2, d, s);
            }

            if (!fs.existsSync(dir)) continue;

            const tMd = path.join(dir, 'ticket.md');
            const tJson = path.join(dir, 'ticket.json');

            if (fs.existsSync(tMd)) {
                try {
                    let content = fs.readFileSync(tMd, 'utf8');
                    const fmRegex = /^---\n([\s\S]*?)\n---/;
                    const match = content.match(fmRegex);
                    if (match) {
                        const fm = match[1];
                        for (const [oldId, newId] of oldToNewGoalId.entries()) {
                            const goalRegex = new RegExp(`^goal:\\s*["']?${escapeRegExp(oldId)}["']?$`, 'm');
                            if (goalRegex.test(fm)) {
                                console.log(`  [MD] Updating ${t.slug}: ${oldId} -> ${newId}`);
                                const newFm = fm.replace(goalRegex, `goal: "${newId}"`);
                                content = content.replace(fm, newFm);
                                fs.writeFileSync(tMd, content);
                                break;
                            }
                        }
                    }
                } catch (e) {
                    console.error(`Error updating md for ${t.slug}`, e);
                }
            }

            if (fs.existsSync(tJson)) {
                try {
                    const tContent = fs.readFileSync(tJson, 'utf8');
                    const json = JSON.parse(tContent);
                    if (json.goal && oldToNewGoalId.has(json.goal)) {
                        const newId = oldToNewGoalId.get(json.goal);
                        if (json.goal !== newId) {
                            console.log(`  [JSON] Updating ${t.slug}: ${json.goal} -> ${newId}`);
                            json.goal = newId;
                            fs.writeFileSync(tJson, JSON.stringify(json, null, 2));
                        }
                    }
                } catch (e) {
                    console.error(`Error updating json for ${t.slug}`, e);
                }
            }
        }
    }

    console.log("Hierarchy refactor complete.");
}

run().catch(e => console.error(e));
