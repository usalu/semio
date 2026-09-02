import re, os, sys

def parse_and_split(path, base_catalog_id, base_capability, satellites, base_dirname_new=None):
    """
    satellites: list of dicts: {name, dirname, catalog_id, capability, kinds:set()}
    base gets whatever kinds remain (not in any satellite).
    """
    with open(path, encoding="utf-8") as f:
        content = f.read()
    lines = content.split("\n")

    # locate tag block (before "Feature:")
    feat_idx = next(i for i,l in enumerate(lines) if l.startswith("Feature:"))
    tag_lines = lines[:feat_idx]
    oracle_tag = next(l for l in tag_lines if l.startswith("@oracle-"))
    comparison_tag = next(l for l in tag_lines if l.startswith("@comparison-"))

    # find block boundaries
    def find_tag_line(tag):
        return next(i for i,l in enumerate(lines) if l.strip() == tag)

    mutate_tag_idx = find_tag_line("@id-mutate")
    inverse_tag_idx = find_tag_line("@id-inverse")
    # trailer tag: next "  @id-" after inverse block, else EOF
    trailer_tag_idx = None
    for i in range(inverse_tag_idx+1, len(lines)):
        if lines[i].strip().startswith("@id-"):
            trailer_tag_idx = i
            break

    description_lines = lines[feat_idx:mutate_tag_idx]  # includes Feature: line + blank + paragraphs, up to (not incl) mutate tag block

    def parse_outline_block(start_tag_idx, end_idx):
        # block = lines[start_tag_idx:end_idx]
        block = lines[start_tag_idx:end_idx]
        # find "Examples:" line index within block
        ex_idx = next(i for i,l in enumerate(block) if l.strip() == "Examples:")
        head = block[:ex_idx+1]  # tags + Scenario Outline + steps + "Examples:" line
        rest = block[ex_idx+1:]
        # header row is first non-empty row
        header_row = rest[0]
        data_rows = [r for r in rest[1:] if r.strip().startswith("|")]
        return head, header_row, data_rows

    mutate_head, mutate_header_row, mutate_rows = parse_outline_block(mutate_tag_idx, inverse_tag_idx)
    inverse_end = trailer_tag_idx if trailer_tag_idx is not None else len(lines)
    inverse_head, inverse_header_row, inverse_rows = parse_outline_block(inverse_tag_idx, inverse_end)
    trailer_lines = lines[trailer_tag_idx:] if trailer_tag_idx is not None else []

    def row_id(row):
        # "      | id-value             | params |"
        cell = row.split("|")[1].strip()
        return cell

    all_kinds_in_order = [row_id(r) for r in mutate_rows]

    satellite_kind_sets = {}
    claimed_kinds = set()
    for sat in satellites:
        satellite_kind_sets[sat["name"]] = set(sat["kinds"])
        claimed_kinds |= set(sat["kinds"])
    base_kinds = [k for k in all_kinds_in_order if k not in claimed_kinds]

    def build_file(tags, feature_desc_lines, mtag_lines, mheader, mrows, itag_lines, iheader, irows, trailer):
        out = []
        out.extend(tags)
        out.extend(feature_desc_lines)
        out.extend(mtag_lines)
        out.append(mheader)
        out.extend(mrows)
        out.append("")
        out.extend(itag_lines)
        out.append(iheader)
        out.extend(irows)
        if trailer:
            out.append("")
            out.extend(trailer)
        text = "\n".join(out)
        if not text.endswith("\n"):
            text += "\n"
        return text

    results = {}

    # BASE
    base_tags = [f"@capability-{base_capability}", oracle_tag, comparison_tag, f"@mutations-{base_catalog_id}"]
    base_mrows = [r for r in mutate_rows if row_id(r) in base_kinds]
    base_irows = [r for r in inverse_rows if row_id(r) in base_kinds]
    base_text = build_file(base_tags, description_lines, mutate_head, mutate_header_row, base_mrows,
                            inverse_head, inverse_header_row, base_irows, trailer_lines)
    results["__base__"] = base_text

    # SATELLITES
    for sat in satellites:
        ks = satellite_kind_sets[sat["name"]]
        smrows = [r for r in mutate_rows if row_id(r) in ks]
        sirows = [r for r in inverse_rows if row_id(r) in ks]
        stags = [f"@capability-{sat['capability']}", oracle_tag, comparison_tag, f"@mutations-{sat['catalog_id']}"]
        # short description for satellite (reuse Feature: line, replace body with pointer note)
        feat_line = description_lines[0]
        short_desc = [feat_line, f"  See ../{os.path.basename(os.path.dirname(path))}/🥒️.feature for the full fixture/provenance narrative -- this subset's own scenarios exercise only the mutation kinds `../../🏅️standards` places under this subset.", ""]
        stext = build_file(stags, short_desc, mutate_head, mutate_header_row, smrows,
                            inverse_head, inverse_header_row, sirows, trailer=None)
        results[sat["name"]] = stext

    return results, base_kinds, all_kinds_in_order

if __name__ == "__main__":
    pass
