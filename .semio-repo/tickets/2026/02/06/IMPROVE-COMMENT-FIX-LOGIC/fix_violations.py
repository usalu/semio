import re

with open("/workspaces/semio/semio-repo/cli/main.go", "r") as f:
    content = f.read()

# Update CreateViolation definition
content = re.sub(
    r"func \(ctx \*PolicyContext\) CreateViolation\(summary string, kind ViolationKind, scope string, line int, excerpt string\) Violation \{",
    r"func (ctx *PolicyContext) CreateViolation(summary string, kind ViolationKind, scope string, line int, col int, excerpt string) Violation {",
    content,
)

# Update BuildID in CreateViolation
content = re.sub(
    r"ID:\s+buildViolationID\(scope, line, 0\),",
    r"ID:      buildViolationID(scope, line, col),",
    content,
)

# Update Column assignment in CreateViolation
content = re.sub(r"Line:\s+line,", r"Line:    line,\n\t\tColumn:  col,", content)

# Update all calls to CreateViolation
# Patterns like ctx.CreateViolation(summary, kind, file, line, "")
# We want to insert 0 before the last argument (excerpt)

# This regex finds ctx.CreateViolation( followed by 4 arguments separated by commas,
# and the last argument.
# We need to be careful with nested parentheses.
# Given the usage, they are usually on one or more lines.


def replace_call(match):
    prefix = match.group(1)
    args_str = match.group(2)
    # Split arguments by comma, but respect nested parens/quotes
    # For now, let's assume simple cases or handle with care.
    # Actually, most calls have 5 arguments. We want to make it 6 by adding 0.

    # Let's count commas.
    commas = args_str.count(",")
    if commas == 4:
        # It has 5 args. Insert 0 before the 5th (last).
        parts = args_str.rsplit(",", 1)
        return f"{prefix}({parts[0]}, 0, {parts[1]})"
    return match.group(0)


# Pattern: ctx.CreateViolation( ... )
# We use a non-greedy match for the arguments
# But wait, arguments can contain commas inside quotes.
# Let's use a simpler approach: finding the last comma.

# Regex to find ctx.CreateViolation( followed by balanced parentheses
# This is hard with regex.

# Let's just use a simpler replacement for the known 5-arg calls.
# ctx.CreateViolation(val1, val2, val3, val4, val5) -> ctx.CreateViolation(val1, val2, val3, val4, 0, val5)

content = re.sub(
    r"(ctx\.CreateViolation\([^,]+,[^,]+,[^,]+,[^,]+),", r"\1, 0,", content
)

with open("/workspaces/semio/semio-repo/cli/main.go", "w") as f:
    f.write(content)
