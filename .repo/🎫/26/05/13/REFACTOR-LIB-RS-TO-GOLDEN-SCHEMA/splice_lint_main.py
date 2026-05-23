# Temporary splice helper for repo/client/cli/main.go (lint-script migration).
from pathlib import Path

ROOT = Path(__file__).resolve().parents[6]  # .../semio
MAIN = ROOT / "repo" / "client" / "cli" / "main.go"


def brace_slice(text: str, open_idx: int) -> tuple[str, int]:
    """Return (content inside first {...}, index after closing }). open_idx points at '{'."""
    depth = 0
    i = open_idx
    while i < len(text):
        c = text[i]
        if c == "{":
            depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0:
                return text[open_idx + 1 : i], i + 1
        i += 1
    raise RuntimeError("unbalanced brace")


def main() -> None:
    text = MAIN.read_text(encoding="utf-8")

    # 1) Empty policies slice
    marker = "var policies = []PolicyDef{"
    i = text.index(marker)
    j = i + len(marker)
    _, j = brace_slice(text, j - 1)  # j-1 points at '{'
    text = text[:i] + "var policies = []PolicyDef{}\n" + text[j:]

    # 2) Replace CheckPolicies + CheckPoliciesWithContext + matchesScope with stubs (before deleting policy funcs)
    c_start = text.index("// ✔️CheckPolicies MUST run all applicable policies")
    c_end = text.index("// 🟩headerPolicy holds the data fields for a headerPolicy record.", c_start)
    stub = """// ✔️CheckPolicies is a legacy no-op; breachs come from lint scripts and `.repo/cache/breaches`.
func CheckPolicies(scope Scope, bundles []Bundle, policyIDs []string) ([]Breach, error) {
	return nil, nil
}

// 🟨CheckPoliciesWithContext is a legacy no-op; breachs come from lint scripts and `.repo/cache/breaches`.
func CheckPoliciesWithContext(ctx *PolicyContext, policyIDs []string) ([]Breach, error) {
	return nil, nil
}

"""
    text = text[:c_start] + stub + text[c_end:]

    # 3) Remove headerPolicy … semioPolicy block (keep #endregion 🧊Policies)
    start = text.index("// 🟩headerPolicy holds the data fields for a headerPolicy record.")
    end = text.index("// #endregion 🧊Policies", start)
    text = text[:start] + "\n" + text[end:]

    # 4) Remove applyAutofixes through findMatchingSectionStartName (before TicketOpen)
    a_start = text.index("func applyAutofixes(file string, breachs []Breach) (int, error) {")
    a_end = text.index("// 📬TicketOpen MUST return a non-nil error", a_start)
    text = text[:a_start] + text[a_end:]

    MAIN.write_text(text, encoding="utf-8", newline="\n")
    print("splice ok", MAIN)


if __name__ == "__main__":
    main()
