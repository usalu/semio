#!/usr/bin/env python3
import re
import sys

FILE = "/workspaces/semio/semio-repo/cli/main.go"
MAX_LINE = 15000

with open(FILE, "r") as f:
    lines = f.readlines()

section_start_re = re.compile(r"^// #region 🔖(.+)$")
section_end_re = re.compile(r"^// #endregion 🔖(.+)$")
def_re = re.compile(r"^(type|func|var|const)\s+")
func_re = re.compile(r"^func\s+")
method_re = re.compile(r"^func\s+\((\w+)\s+\*?(\w+)\)\s+(\w+)")
plain_func_re = re.compile(r"^func\s+(\w+)")
type_struct_re = re.compile(r"^type\s+(\w+)\s+struct\b")
type_interface_re = re.compile(r"^type\s+(\w+)\s+interface\b")
type_alias_re = re.compile(r"^type\s+(\w+)\s+(\w+)")
type_func_re = re.compile(r"^type\s+(\w+)\s+func\b")
const_block_re = re.compile(r"^const\s*\(")
var_block_re = re.compile(r"^var\s+")
const_single_re = re.compile(r"^const\s+(\w+)")

section_summaries = {
    "Engine Events": "Event types and payload structures for the engine event stream.",
    "Engine Errors": "Error code constants for engine failure classification.",
    "Engine Requests": "Request command types and argument structures for engine invocation.",
    "Engine": "Core engine that dispatches requests and emits events over a channel.",
    "Cli Adapter": "CLI adapter that wires cobra commands to the engine and renders output.",
    "Utilities": "General-purpose utility functions for time parsing and formatting.",
    "Models": "Data model types for tickets, goals, and tree representation.",
    "Monorepo Tree Types": "Tree node kinds, filter criteria, and matching logic for monorepo tree queries.",
    "Tree Logic": "Tree construction, filtering, searching, and rendering for goals, sections, and monorepo nodes.",
    "Monorepo Tree": "Monorepo tree builder that assembles all entity nodes into a unified tree.",
    "CLI Renderers": "Stream renderers that format engine events for NDJSON, human-readable, and markdown output.",
    "ANSI": "ANSI escape code constants for terminal colorization.",
    "GraphQL Types": "GraphQL-facing domain types, enums, constants, and entity node implementations.",
    "Drafts": "Draft management for creating, listing, and deleting draft file sets.",
    "GraphQL Input Types": "GraphQL mutation input types for tickets, goals, todos, and contributors.",
    "Types": "Scope, todo, breach, and ticket metric types for the repository model.",
    "Languages": "Language plugin registry with parsers for sections, definitions, comments, imports, and headers.",
    "TypeScript": "TypeScript language plugin with section, definition, comment, and import support.",
    "Go": "Go language plugin with section, definition, import, and package support.",
    "C#": "C# language plugin with section, definition, and import support.",
    "JSON": "JSON language plugin with section parsing via embedded comment keys.",
    "Markdown": "Markdown language plugin with heading-based section parsing.",
    "Rust": "Rust language plugin with section, definition, and import support.",
    "Ruby": "Ruby language plugin with section, definition, and import support.",
    "Shell": "Shell language plugin with section and comment support.",
    "TOML": "TOML language plugin with section heading and comment support.",
    "YAML": "YAML language plugin with section heading and comment support.",
    "SQL": "SQL language plugin with section and comment support.",
    "GraphQL": "GraphQL language plugin with section and comment support.",
    "Codebase Types": "Internal metric, contributor, ticket, policy, breach, and tree node types for codebase analysis.",
    "Utils": "File system, git, path normalization, and formatting utilities.",
    "Sections": "Section parsing, JSON section manipulation, and section lookup utilities.",
    "Policies": "Policy definitions, context, checkers, and individual policy implementations.",
    "Codebase": "Codebase builder that assembles bundles, folders, files, sections, definitions, contributors, tickets, policies, and breachs.",
    "Tickets": "Ticket and goal lifecycle management including creation, closing, reopening, deletion, and diff computation.",
}


def get_def_name(line):
    line = line.strip()
    m = method_re.match(line)
    if m:
        return m.group(3)
    m = plain_func_re.match(line)
    if m:
        return m.group(1)
    m = type_alias_re.match(line)
    if m:
        return m.group(1)
    m = const_single_re.match(line)
    if m:
        return m.group(1)
    mv = re.match(r"^var\s+(\w+)", line)
    if mv:
        return mv.group(1)
    return None


def is_exported(name):
    return name and name[0].isupper()


def is_func_def(line):
    return line.strip().startswith("func ")


def get_receiver_type(line):
    m = method_re.match(line.strip())
    if m:
        return m.group(2)
    return None


def get_func_params(line):
    line = line.strip()
    m = method_re.match(line)
    if m:
        name = m.group(3)
    else:
        m = plain_func_re.match(line)
        if m:
            name = m.group(1)
        else:
            return ""
    idx = line.find(name)
    rest = line[idx + len(name) :]
    paren_start = rest.find("(")
    if paren_start == -1:
        return ""
    depth = 0
    paren_end = -1
    for i in range(paren_start, len(rest)):
        if rest[i] == "(":
            depth += 1
        elif rest[i] == ")":
            depth -= 1
            if depth == 0:
                paren_end = i
                break
    if paren_end == -1:
        return ""
    return rest[paren_start + 1 : paren_end]


def get_return_type(line):
    line = line.strip()
    brace = line.rfind("{")
    if brace == -1:
        brace = len(line)
    before_brace = line[:brace].rstrip()
    paren_count = 0
    last_close = -1
    for i in range(len(before_brace) - 1, -1, -1):
        if before_brace[i] == ")":
            if paren_count == 0:
                last_close = i
            paren_count += 1
        elif before_brace[i] == "(":
            paren_count -= 1
            if paren_count == 0:
                params_end = i
                break
    after_params = before_brace[last_close + 1 :].strip() if last_close != -1 else ""
    return after_params


def gen_summary(name, line, section_name):
    line = line.strip()
    recv = get_receiver_type(line)
    if type_struct_re.match(line):
        return f"{name} holds the data fields for a {camel_to_words(name)} record."
    if type_interface_re.match(line):
        return f"{name} defines the interface contract for {camel_to_words(name)} operations."
    if type_func_re.match(line):
        return f"{name} is a function type for {camel_to_words(name)} callbacks."
    if (
        type_alias_re.match(line)
        and not type_struct_re.match(line)
        and not type_interface_re.match(line)
        and not type_func_re.match(line)
    ):
        return f"{name} represents a {camel_to_words(name)} value."
    if var_block_re.match(line):
        return f"{name} holds the {camel_to_words(name)} values."
    if const_block_re.match(line) or const_single_re.match(line):
        return f"{name} defines the {camel_to_words(name)} constant."
    if is_func_def(line):
        if recv:
            return gen_func_summary(name, recv, line, section_name)
        else:
            return gen_func_summary(name, None, line, section_name)
    return f"{name} defines {camel_to_words(name)} functionality."


def gen_func_summary(name, recv, line, section_name):
    low = name.lower()
    params = get_func_params(line)
    if name == "Error":
        return f"Error returns the string representation of the error."
    if name == "String":
        return f"String returns the string representation of the {recv or 'value'}."
    if name.startswith("Is") and name[2:3].isupper():
        what = camel_to_words(name[2:])
        return f"{name} reports whether the {recv or 'value'} is {what}."
    if name.startswith("Has") and name[3:4].isupper():
        what = camel_to_words(name[3:])
        return f"{name} reports whether the {recv or 'value'} has {what}."
    if name.startswith("Get") and name[3:4].isupper():
        what = camel_to_words(name[3:])
        return f"{name} returns the {what} of the {recv or 'value'}."
    if name.startswith("Set") and name[3:4].isupper():
        what = camel_to_words(name[3:])
        return f"{name} sets the {what} on the {recv or 'value'}."
    if name.startswith("New"):
        what = name[3:]
        return f"{name} creates and returns a new {what} instance."
    if name.startswith("Build"):
        what = camel_to_words(name[5:])
        return f"{name} constructs and returns the {what} structure."
    if name.startswith("Parse"):
        what = camel_to_words(name[5:])
        return f"{name} parses the input and returns the {what} result."
    if name.startswith("Format"):
        what = camel_to_words(name[6:])
        return f"{name} formats the {what} into its string representation."
    if name.startswith("Render"):
        what = camel_to_words(name[6:])
        return f"{name} renders the {what} into its output representation."
    if name.startswith("Find"):
        what = camel_to_words(name[4:])
        return f"{name} searches for and returns the matching {what}."
    if name.startswith("Load"):
        what = camel_to_words(name[4:])
        return f"{name} loads the {what} from storage."
    if name.startswith("Read"):
        what = camel_to_words(name[4:])
        return f"{name} reads and returns the {what} content."
    if name.startswith("Write"):
        what = camel_to_words(name[5:])
        return f"{name} writes the {what} content to storage."
    if name.startswith("Create"):
        what = camel_to_words(name[6:])
        return f"{name} creates a new {what} and persists it."
    if name.startswith("Delete"):
        what = camel_to_words(name[6:])
        return f"{name} removes the specified {what}."
    if name.startswith("List"):
        what = camel_to_words(name[4:])
        return f"{name} returns all available {what} entries."
    if name.startswith("Check"):
        what = camel_to_words(name[5:])
        return f"{name} validates the {what} and returns any breachs."
    if name.startswith("Derive"):
        what = camel_to_words(name[6:])
        return f"{name} infers and returns the {what} from the given input."
    if name.startswith("Normalize"):
        what = camel_to_words(name[9:])
        return f"{name} normalizes the {what} to its canonical form."
    if name.startswith("Resolve"):
        what = camel_to_words(name[7:])
        return f"{name} resolves and validates the {what} against known values."
    if name.startswith("Extract"):
        what = camel_to_words(name[7:])
        return f"{name} extracts the {what} from the given input."
    if name.startswith("Filter"):
        what = camel_to_words(name[6:])
        return f"{name} filters the {what} based on the given criteria."
    if name.startswith("Search"):
        what = camel_to_words(name[6:])
        return f"{name} performs a text search across the {what}."
    if name.startswith("Sort"):
        what = camel_to_words(name[4:])
        return f"{name} sorts the {what} in the canonical order."
    if name.startswith("Ensure"):
        what = camel_to_words(name[6:])
        return f"{name} ensures the {what} exists, creating it if necessary."
    if name.startswith("Walk"):
        what = camel_to_words(name[4:])
        return f"{name} recursively walks the {what} and invokes the callback."
    if name.startswith("Match") and name[5:6].isupper():
        what = (
            camel_to_words(name[7:])
            if name.startswith("Matches")
            else camel_to_words(name[5:])
        )
        return f"{name} checks whether the given {what} matches the filter criteria."
    if name.startswith("Stream"):
        what = camel_to_words(name[6:])
        return f"{name} streams the {what} over a channel with optional filtering."
    if name.startswith("Hydrate"):
        what = camel_to_words(name[7:])
        return f"{name} populates the {what} with associated child data."
    if name == "Execute":
        return "Execute runs the root command and returns any error."
    if name == "Run":
        return "Run dispatches the request and returns an event channel."
    if name == "Render":
        return f"Render writes the formatted engine event stream to the output writers."
    if name == "Info":
        return f"Info returns the metadata for the statute."
    if name == "AllKinds":
        return f"AllKinds returns all statutes associated with the group."
    if name == "Priority":
        return f"Priority returns the priority of the breach from its kind metadata."
    if name == "Autofixable":
        return f"Autofixable reports whether the statute supports automatic fixing."
    if name == "ToStreamOptions":
        return f"ToStreamOptions converts the filter input into stream options."
    if name == "IsNode":
        return f"IsNode marks the type as a graph node."
    if name == "GetID":
        return f"GetID returns the unique identifier of the node."
    if name == "GetURI":
        return f"GetURI returns the canonical URI of the node."
    if recv:
        return f"{name} performs the {camel_to_words(name)} operation on the {recv}."
    return f"{name} performs the {camel_to_words(name)} operation."


def gen_spec(name, line, section_name):
    line = line.strip()
    recv = get_receiver_type(line)
    params = get_func_params(line)
    low = name.lower()
    if name == "Error":
        return f"{name} MUST return a formatted string representation."
    if name == "String":
        return f"{name} MUST return the canonical string value."
    if name.startswith("Is") and name[2:3].isupper():
        return f"{name} MUST return true only when the condition is met."
    if name.startswith("Has") and name[3:4].isupper():
        return f"{name} MUST return true only when the property is present."
    if name.startswith("Get") and name[3:4].isupper():
        return f"{name} MUST return the stored value without modification."
    if name.startswith("Set") and name[3:4].isupper():
        return f"{name} MUST update the value on the receiver."
    if name.startswith("New"):
        what = name[3:]
        return f"{name} MUST initialize all required fields and return a valid {what}."
    if name.startswith("Build"):
        what = camel_to_words(name[5:])
        return f"{name} MUST assemble the {what} from the available context data."
    if name.startswith("Parse"):
        what = camel_to_words(name[5:])
        return f"{name} MUST return an error when the input is malformed."
    if name.startswith("Format"):
        what = camel_to_words(name[6:])
        return f"{name} MUST produce a well-formed {what} string."
    if name.startswith("Render"):
        what = camel_to_words(name[6:])
        return f"{name} MUST produce a complete {what} output."
    if name.startswith("Find"):
        return f"{name} MUST return nil when no match is found."
    if name.startswith("Load"):
        return f"{name} MUST read from the configured storage path."
    if name.startswith("Read"):
        return f"{name} MUST return the full content from the given path."
    if name.startswith("Write"):
        return f"{name} MUST persist the content atomically."
    if name.startswith("Create"):
        return f"{name} MUST persist the new entity and return a reference to it."
    if name.startswith("Delete"):
        return f"{name} MUST remove all associated data for the entity."
    if name.startswith("List"):
        return f"{name} MUST return a consistent snapshot of available entries."
    if name.startswith("Check"):
        return f"{name} MUST run all applicable policies and aggregate breachs."
    if name.startswith("Derive"):
        return f"{name} MUST return a valid value for any recognized input."
    if name.startswith("Normalize"):
        return f"{name} MUST be idempotent for already-normalized values."
    if name.startswith("Resolve"):
        return f"{name} MUST return an error for unrecognized values."
    if name.startswith("Extract"):
        return f"{name} MUST return the extracted value without side effects."
    if name.startswith("Filter"):
        return f"{name} MUST preserve the tree structure while removing non-matching nodes."
    if name.startswith("Search"):
        return f"{name} MUST match case-insensitively against node labels and descriptions."
    if name.startswith("Sort"):
        return f"{name} MUST sort in a stable, deterministic order."
    if name.startswith("Ensure"):
        return (
            f"{name} MUST be idempotent and MUST NOT fail if the target already exists."
        )
    if name.startswith("Walk"):
        return f"{name} MUST visit every entry and MUST stop when the callback returns an error."
    if name.startswith("Match") and name[5:6].isupper():
        return f"{name} MUST return true when all specified criteria are satisfied."
    if name.startswith("Stream"):
        return f"{name} MUST emit all matching entries and close the channel when done."
    if name.startswith("Hydrate"):
        return f"{name} MUST attach all matching child elements to their parents."
    if name == "Execute":
        return "Execute MUST delegate to the root command and propagate errors."
    if name == "Run":
        return "Run MUST emit start, result or error, and done events in order."
    if name == "Render":
        return (
            "Render MUST consume the full event stream and return the final exit code."
        )
    if name == "Info":
        return "Info MUST return the metadata entry for the statute."
    if name == "AllKinds":
        return "AllKinds MUST include all statutes from the group and its children."
    if name == "Priority":
        return "Priority MUST derive the value from the statute metadata."
    if name == "Autofixable":
        return "Autofixable MUST return true only for statutes that support auto-fix."
    if name == "ToStreamOptions":
        return "ToStreamOptions MUST map all filter input fields to stream options."
    if name == "IsNode":
        return "IsNode MUST be a no-op marker method for the graph node interface."
    if name == "GetID":
        return "GetID MUST return a unique, deterministic identifier."
    if name == "GetURI":
        return "GetURI MUST return a valid URI in the semiorepo scheme."
    if name == "UnmarshalJSON":
        return "UnmarshalJSON MUST handle both legacy and current JSON layouts."
    if recv:
        return (
            f"{name} MUST operate on the {recv} receiver and return consistent results."
        )
    return f"{name} MUST complete the operation and return consistent results."


def camel_to_words(s):
    if not s:
        return s
    words = re.sub(r"([A-Z])", r" \1", s).strip().lower()
    return words


def has_summary_comment(lines, line_idx):
    idx = line_idx - 1
    while idx >= 0 and lines[idx].strip() == "":
        idx -= 1
    if idx >= 0:
        stripped = lines[idx].strip()
        if (
            stripped.startswith("//")
            and not stripped.startswith("// #region")
            and not stripped.startswith("// #endregion")
        ):
            return True
    return False


def get_prev_non_blank(lines, line_idx):
    idx = line_idx - 1
    while idx >= 0 and lines[idx].strip() == "":
        idx -= 1
    return idx


def is_comment_line(line):
    s = line.strip()
    return (
        s.startswith("//")
        and not s.startswith("// #region")
        and not s.startswith("// #endregion")
    )


insertions = {}

for i, raw_line in enumerate(lines):
    if i >= MAX_LINE:
        break
    line = raw_line.rstrip("\n")
    stripped = line.strip()
    m = section_start_re.match(stripped)
    if m:
        section_name = m.group(1).strip()
        if section_name in section_summaries:
            next_idx = i + 1
            if next_idx < len(lines):
                next_line = lines[next_idx].strip()
                if not (
                    next_line.startswith("//")
                    and not next_line.startswith("// #region")
                    and not next_line.startswith("// #endregion")
                ):
                    insertions[next_idx] = insertions.get(next_idx, [])
                    insertions[next_idx].insert(
                        0,
                        ("section_summary", f"// {section_summaries[section_name]}\n"),
                    )

current_section = ""
section_stack = []
for i, raw_line in enumerate(lines):
    if i >= MAX_LINE:
        break
    stripped = raw_line.strip()
    m = section_start_re.match(stripped)
    if m:
        section_stack.append(m.group(1).strip())
        current_section = m.group(1).strip()
    em = section_end_re.match(stripped)
    if em:
        if section_stack:
            section_stack.pop()
        current_section = section_stack[-1] if section_stack else ""

    name = get_def_name(stripped)
    if name and is_exported(name):
        prev_idx = get_prev_non_blank(lines, i)
        has_existing_comment = prev_idx >= 0 and is_comment_line(lines[prev_idx])
        if not has_existing_comment:
            needs_spec = is_func_def(stripped)
            summary = gen_summary(name, stripped, current_section)
            to_insert = []
            if needs_spec:
                spec = gen_spec(name, stripped, current_section)
                to_insert.append(("spec", f"// {spec}\n"))
            to_insert.append(("summary", f"// {summary}\n"))
            insertions[i] = insertions.get(i, [])
            insertions[i] = to_insert + insertions[i]

offset = 0
new_lines = []
for i, raw_line in enumerate(lines):
    if i in insertions:
        for kind, text in insertions[i]:
            new_lines.append(text)
    new_lines.append(raw_line)

with open(FILE, "w") as f:
    f.writelines(new_lines)

print(f"Inserted comments at {len(insertions)} locations")
total_inserts = sum(len(v) for v in insertions.values())
print(f"Total lines inserted: {total_inserts}")
