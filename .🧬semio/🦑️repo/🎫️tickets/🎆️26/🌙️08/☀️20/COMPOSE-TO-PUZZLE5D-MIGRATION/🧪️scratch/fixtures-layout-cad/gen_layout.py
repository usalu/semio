#!/usr/bin/env python3
"""Emit the 25 handcrafted layout mutation fixtures. Every case's before/after/mutation/outcome
and every assertion body is written out individually below — nothing is derived from a mutation's
name."""
import copy, json, os, pathlib, re

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
MUT = ROOT / "✏️s/\U0001f50c️plugins/\U0001f4cf️layout/\U0001f5ff️artifacts/\U0001f4cf️layout/\U0001f3c5️standards/\U0001f516️1/\U0001fa86️subsets/✳️any/\U0001f9ec️schema/\U0001f9ec️mutations"

RECT = {
    "kind": "rect", "id": "frame-rect", "layerId": "layer-1",
    "bounds": {"x": 20.0, "y": 30.0, "w": 60.0, "h": 40.0, "rotation": 0.0},
    "locked": None, "visible": None, "fill": [1.0, 1.0, 1.0, 1.0], "stroke": None,
}
TEXT = {
    "kind": "text", "id": "frame-text", "layerId": "layer-1",
    "bounds": {"x": 20.0, "y": 90.0, "w": 160.0, "h": 120.0, "rotation": 0.0},
    "locked": None, "visible": None, "storyId": "story-1", "threadNext": None, "columns": 1,
    "inset": {"x": 0.0, "y": 0.0, "w": 0.0, "h": 0.0}, "wrapMode": "box",
}
BADGE = {
    "kind": "rect", "id": "frame-badge", "layerId": "layer-1",
    "bounds": {"x": 120.0, "y": 30.0, "w": 40.0, "h": 40.0, "rotation": 0.0},
    "locked": None, "visible": None, "fill": [0.0, 0.5, 1.0, 1.0], "stroke": None,
}

PAGE_1 = {
    "id": "page-1", "name": "Cover", "spreadId": "spread-1", "parentPageId": None,
    "width": 200.0, "height": 300.0,
    "margins": {"top": 10.0, "right": 10.0, "bottom": 10.0, "left": 10.0},
    "columns": {"count": 1, "gutter": 0.0},
    "guides": [], "layerIds": ["layer-1"],
    "layers": [{"id": "layer-1", "name": "Content", "visible": True, "locked": False,
                "objectIds": ["frame-rect", "frame-text"]}],
    "frames": [RECT, TEXT], "overrides": [],
}
PAGE_2 = {
    "id": "page-2", "name": "Spare", "spreadId": "spread-1", "parentPageId": None,
    "width": 200.0, "height": 300.0,
    "margins": {"top": 10.0, "right": 10.0, "bottom": 10.0, "left": 10.0},
    "columns": {"count": 1, "gutter": 0.0},
    "guides": [], "layerIds": ["layer-2"],
    "layers": [{"id": "layer-2", "name": "Content", "visible": True, "locked": False, "objectIds": []}],
    "frames": [], "overrides": [],
}
PAGE_3 = {
    "id": "page-3", "name": "Back", "spreadId": "spread-2", "parentPageId": None,
    "width": 200.0, "height": 300.0,
    "margins": {"top": 5.0, "right": 5.0, "bottom": 5.0, "left": 5.0},
    "columns": {"count": 2, "gutter": 6.0},
    "guides": [], "layerIds": ["layer-3"],
    "layers": [{"id": "layer-3", "name": "Content", "visible": True, "locked": False, "objectIds": []}],
    "frames": [], "overrides": [],
}

BASE = {
    "schema": "layout.layout",
    "name": "Fixture Layout",
    "grid": {"baselineGrid": 12.0, "baselineOffset": 0.0, "snapToBaseline": True},
    "paragraphStyles": [],
    "characterStyles": [],
    "stories": [
        {"id": "story-1", "content": "Alpha body.", "styleRuns": []},
        {"id": "story-2", "content": "Spare body.", "styleRuns": []},
    ],
    "links": [
        {"id": "link-1", "path": "alpha.png", "hash": "hash-alpha", "width": 800, "height": 600,
         "dpi": 300, "colorProfile": None, "state": None, "proxyDataUrl": None},
        {"id": "link-2", "path": "spare.png", "hash": "hash-spare", "width": 400, "height": 300,
         "dpi": 72, "colorProfile": None, "state": None, "proxyDataUrl": None},
    ],
    "parentPages": [],
    "spreads": [],
    "pages": [PAGE_1, PAGE_2],
    "printTarget": None,
}

STORY_3 = {"id": "story-3", "content": "Caption.", "styleRuns": []}
LINK_3 = {"id": "link-3", "path": "caption.png", "hash": "hash-caption", "width": 200,
          "height": 150, "dpi": 144, "colorProfile": None, "state": None, "proxyDataUrl": None}

APPLIED = {"status": "applied"}

def base():
    return copy.deepcopy(BASE)

#region transforms — each one is the hand-application of exactly what that mutation's diff builder does
def t_rename_layout(s):
    s["name"] = "Renamed Fixture"

def t_change_print_target(s):
    s["printTarget"] = "cmyk-coated"

def t_change_data_fields(s):
    s["dataFieldsJson"] = "{\"client\":\"acme\"}"

def t_create_page(s):
    s["pages"].append(copy.deepcopy(PAGE_3))

def t_delete_page(s):
    s["pages"] = [p for p in s["pages"] if p["id"] != "page-2"]

def t_rename_page(s):
    s["pages"][0]["name"] = "Title Page"

def t_change_page_width(s):
    s["pages"][0]["width"] = 240.0

def t_change_page_height(s):
    s["pages"][0]["height"] = 360.0

def t_update_page_margins(s):
    s["pages"][0]["margins"] = {"top": 12.0, "right": 18.0, "bottom": 24.0, "left": 6.0}

def t_update_page_columns(s):
    s["pages"][0]["columns"] = {"count": 3, "gutter": 12.0}

def t_reorder_pages(s):
    s["pages"] = [s["pages"][1], s["pages"][0]]

def t_create_story(s):
    s["stories"].append(copy.deepcopy(STORY_3))

def t_delete_story(s):
    s["stories"] = [x for x in s["stories"] if x["id"] != "story-2"]

def t_edit_story(s):
    s["stories"][0]["content"] = "Alpha body, revised."

def t_create_link(s):
    s["links"].append(copy.deepcopy(LINK_3))

def t_delete_link(s):
    s["links"] = [x for x in s["links"] if x["id"] != "link-2"]

def t_change_link_path(s):
    s["links"][0]["path"] = "alpha-v2.png"

def t_create_frame(s):
    page = s["pages"][0]
    page["frames"].insert(1, copy.deepcopy(BADGE))
    page["layers"][0]["objectIds"].append("frame-badge")

def t_delete_frame(s):
    page = s["pages"][0]
    page["frames"] = [f for f in page["frames"] if f["id"] != "frame-text"]
    page["layers"][0]["objectIds"] = [i for i in page["layers"][0]["objectIds"] if i != "frame-text"]

def t_move_frame(s):
    b = s["pages"][0]["frames"][0]["bounds"]
    b["x"], b["y"] = 55.0, 65.0

def t_resize_frame(s):
    b = s["pages"][0]["frames"][0]["bounds"]
    b["w"], b["h"] = 90.0, 70.0

def t_change_frame_fill(s):
    s["pages"][0]["frames"][0]["fill"] = [0.5, 0.25, 0.75, 1.0]

def t_change_frame_stroke(s):
    s["pages"][0]["frames"][0]["stroke"] = [0.0, 0.0, 0.0, 1.0]

def t_change_frame_wrap_mode(s):
    s["pages"][0]["frames"][1]["wrapMode"] = "column"

def t_change_frame_columns(s):
    s["pages"][0]["frames"][1]["columns"] = 2
#endregion

CASES = [
    dict(
        leaf="rename-layout", kind="rename-layout", case="renames-the-document",
        mod="tests_renames_the_document", transform=t_rename_layout,
        mutation={"RenameLayout": {"new_name": "Renamed Fixture"}},
        blurb="Proves the document-root `name` scalar is the only thing `rename-layout` touches.",
        fn1="rewrites_only_the_document_name", fn1doc="▶️ `rename-layout` replaces the root `name` and leaves every collection alone.",
        change="""    assert_eq!(after.name, "Renamed Fixture", "rename-layout must set the document name to the payload's new_name");
    assert_eq!(after.pages.len(), 2, "rename-layout must not add or drop pages");
    assert_eq!(after.stories.len(), 2, "rename-layout must not touch the stories collection");
    assert!(after.print_target.is_none(), "rename-layout must not touch the print target");""",
        fn2="inverse_renames_back_to_fixture_layout", fn2doc="↩️ The inverse is a `rename-layout` carrying the BASE name captured before the edit.",
        inverse="""    assert_eq!(inverse.len(), 1, "rename-layout inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::RenameLayout(step) => assert_eq!(step.new_name, "Fixture Layout", "the inverse must carry the pre-edit document name"),
        other => panic!("rename-layout must invert to rename-layout, got {other:?}"),
    }""",
        probe="""    assert_eq!(produced.diff().name.as_deref(), Some("Renamed Fixture"), "rename-layout fills the diff's root `name` field");
    assert!(produced.diff().pages.is_none(), "rename-layout leaves the pages delta empty");""",
    ),
    dict(
        leaf="change-print-target", kind="change-print-target", case="sets-a-cmyk-print-target",
        mod="tests_sets_a_cmyk_print_target", transform=t_change_print_target,
        mutation={"ChangePrintTarget": {"new_print_target": "cmyk-coated"}},
        blurb="Proves the nullable `print_target` scalar goes from cleared to set.",
        fn1="fills_the_previously_cleared_print_target", fn1doc="▶️ `change-print-target` writes the nullable root scalar and nothing else.",
        change="""    assert_eq!(after.print_target.as_deref(), Some("cmyk-coated"), "change-print-target must set the document print target");
    assert_eq!(after.name, "Fixture Layout", "change-print-target must not rename the document");
    assert!(after.data_fields_json.is_none(), "change-print-target must not touch the data-fields payload");""",
        fn2="inverse_clears_the_print_target_again", fn2doc="↩️ The inverse re-clears the slot, because BASE had no print target.",
        inverse="""    assert_eq!(inverse.len(), 1, "change-print-target inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::ChangePrintTarget(step) => assert!(step.new_print_target.is_none(), "the inverse must carry BASE's cleared print target"),
        other => panic!("change-print-target must invert to change-print-target, got {other:?}"),
    }""",
        probe="""    assert_eq!(produced.diff().print_target, Some(Some("cmyk-coated".to_string())), "change-print-target fills the doubly-optional `print_target` diff field");
    assert!(produced.diff().data_fields_json.is_none(), "change-print-target leaves `data_fields_json` untouched in the diff");""",
    ),
    dict(
        leaf="change-data-fields", kind="change-data-fields", case="attaches-a-data-fields-payload",
        mod="tests_attaches_a_data_fields_payload", transform=t_change_data_fields,
        mutation={"ChangeDataFields": {"new_json": "{\"client\":\"acme\"}"}},
        blurb="Proves the opaque `data_fields_json` blob is replaced wholesale.",
        fn1="stores_the_opaque_json_blob_verbatim", fn1doc="▶️ `change-data-fields` stores the payload string byte-for-byte, unparsed.",
        change="""    assert_eq!(after.data_fields_json.as_deref(), Some("{\\"client\\":\\"acme\\"}"), "change-data-fields must store the payload JSON string verbatim");
    assert!(after.print_target.is_none(), "change-data-fields must not touch the print target");
    assert_eq!(after.name, "Fixture Layout", "change-data-fields must not rename the document");""",
        fn2="inverse_clears_the_data_fields_payload", fn2doc="↩️ The inverse restores BASE's absent payload, i.e. clears the field.",
        inverse="""    assert_eq!(inverse.len(), 1, "change-data-fields inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::ChangeDataFields(step) => assert!(step.new_json.is_none(), "the inverse must carry BASE's absent data-fields payload"),
        other => panic!("change-data-fields must invert to change-data-fields, got {other:?}"),
    }""",
        probe="""    assert_eq!(produced.diff().data_fields_json, Some(Some("{\\"client\\":\\"acme\\"}".to_string())), "change-data-fields fills the doubly-optional `data_fields_json` diff field");
    assert!(produced.diff().print_target.is_none(), "change-data-fields leaves `print_target` untouched in the diff");""",
    ),
    dict(
        leaf="create-page", kind="create-page", case="appends-page-3",
        mod="tests_appends_page_3", transform=t_create_page,
        mutation={"CreatePage": {"page": PAGE_3, "index": 2}},
        blurb="Proves a whole `Page` record (margins, columns, layers) enters the id-keyed collection.",
        fn1="brings_a_whole_page_record_into_the_collection", fn1doc="▶️ `create-page` appends the payload's complete `Page`, margins and columns included.",
        change="""    assert_eq!(after.pages.iter().map(|page| page.id.as_str()).collect::<Vec<_>>(), vec!["page-1", "page-2", "page-3"], "create-page appends the new page (the pages delta's `added` always pushes at the end)");
    let created = after.pages.iter().find(|page| page.id == "page-3").expect("create-page inserts page-3");
    assert_eq!(created.name, "Back", "create-page must carry the payload page's name");
    assert_eq!(created.columns.count, 2, "create-page must carry the payload page's column count");
    assert_eq!(created.margins.top, 5.0, "create-page must carry the payload page's margins");""",
        fn2="inverse_deletes_the_page_it_created", fn2doc="↩️ `create-page` always inverts to `delete-page` of the id it minted — it never inspects BASE.",
        inverse="""    assert_eq!(inverse.len(), 1, "create-page inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::DeletePage(step) => assert_eq!(step.id, "page-3", "the inverse must delete the page id create-page minted"),
        other => panic!("create-page must invert to delete-page, got {other:?}"),
    }""",
        probe="""    let delta = produced.diff().pages.as_ref().expect("create-page fills the pages delta");
    assert_eq!(delta.added.len(), 1, "create-page adds exactly one page");
    assert_eq!(delta.added[0].id, "page-3", "create-page's `added` entry is the payload page");
    assert!(delta.removed.is_empty() && delta.patched.is_empty() && delta.reordered.is_none(), "create-page touches only the `added` arm of the pages delta");""",
    ),
    dict(
        leaf="delete-page", kind="delete-page", case="removes-page-2",
        mod="tests_removes_page_2", transform=t_delete_page,
        mutation={"DeletePage": {"id": "page-2"}},
        blurb="Proves a page id leaves the collection and that undo re-materializes the full record.",
        fn1="drops_page_2_and_keeps_page_1_intact", fn1doc="▶️ `delete-page` removes only the addressed page; sibling pages are untouched.",
        change="""    assert_eq!(after.pages.iter().map(|page| page.id.as_str()).collect::<Vec<_>>(), vec!["page-1"], "delete-page must remove page-2 and only page-2");
    assert_eq!(after.pages[0].frames.len(), 2, "delete-page must not disturb the surviving page's frames");
    assert_eq!(after.stories.len(), 2, "delete-page does not cascade into the stories collection");""",
        fn2="inverse_recreates_the_full_page_record", fn2doc="↩️ The inverse is a `create-page` carrying the ENTIRE removed page plus its original index.",
        inverse="""    assert_eq!(inverse.len(), 1, "delete-page inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::CreatePage(step) => {
            assert_eq!(step.page.id, "page-2", "the inverse must recreate the removed page");
            assert_eq!(step.page.layers[0].id, "layer-2", "the inverse must carry the removed page's layers, not a stub");
            assert_eq!(step.index, Some(1), "the inverse must capture the removed page's original index");
        }
        other => panic!("delete-page must invert to create-page, got {other:?}"),
    }""",
        probe="""    let delta = produced.diff().pages.as_ref().expect("delete-page fills the pages delta");
    assert_eq!(delta.removed, vec!["page-2".to_string()], "delete-page's diff carries the id in `removed`");
    assert!(delta.added.is_empty() && delta.patched.is_empty(), "delete-page touches only the `removed` arm of the pages delta");""",
    ),
    dict(
        leaf="rename-page", kind="rename-page", case="renames-page-1",
        mod="tests_renames_page_1", transform=t_rename_page,
        mutation={"RenamePage": {"id": "page-1", "new_name": "Title Page"}},
        blurb="Proves the page-level `name` field is patched without disturbing geometry.",
        fn1="retitles_page_1_without_resizing_it", fn1doc="▶️ `rename-page` patches one page's `name` and leaves width/height/margins alone.",
        change="""    let page = after.pages.iter().find(|page| page.id == "page-1").expect("page-1 survives");
    assert_eq!(page.name, "Title Page", "rename-page must set the addressed page's name");
    assert_eq!((page.width, page.height), (200.0, 300.0), "rename-page must not resize the page");
    assert_eq!(after.pages[1].name, "Spare", "rename-page must not rename sibling pages");""",
        fn2="inverse_restores_the_cover_title", fn2doc="↩️ The inverse is a `rename-page` carrying the name captured from BASE.",
        inverse="""    assert_eq!(inverse.len(), 1, "rename-page inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::RenamePage(step) => {
            assert_eq!(step.id, "page-1", "the inverse must address the same page");
            assert_eq!(step.new_name, "Cover", "the inverse must carry the pre-edit page name");
        }
        other => panic!("rename-page must invert to rename-page, got {other:?}"),
    }""",
        probe="""    let delta = produced.diff().pages.as_ref().expect("rename-page fills the pages delta");
    assert_eq!(delta.patched.len(), 1, "rename-page patches exactly one page");
    assert_eq!(delta.patched[0].id, "page-1", "rename-page's patch entry addresses page-1");
    assert_eq!(delta.patched[0].patch.name.as_deref(), Some("Title Page"), "rename-page fills only the patch's `name` field");
    assert!(delta.patched[0].patch.width.is_none() && delta.patched[0].patch.height.is_none(), "rename-page must not emit a size patch");""",
    ),
    dict(
        leaf="change-page-width", kind="change-page-width", case="widens-page-1",
        mod="tests_widens_page_1", transform=t_change_page_width,
        mutation={"ChangePageWidth": {"id": "page-1", "new_width": 240.0}},
        blurb="Proves the page `width` scalar moves independently of `height`.",
        fn1="widens_page_1_without_changing_its_height", fn1doc="▶️ `change-page-width` is a single-axis setter — `height` must stay put.",
        change="""    let page = after.pages.iter().find(|page| page.id == "page-1").expect("page-1 survives");
    assert_eq!(page.width, 240.0, "change-page-width must set the addressed page's width");
    assert_eq!(page.height, 300.0, "change-page-width must leave the height at its BASE value");
    assert_eq!(after.pages[1].width, 200.0, "change-page-width must not resize sibling pages");""",
        fn2="inverse_narrows_page_1_back_to_200", fn2doc="↩️ The inverse is a `change-page-width` carrying BASE's width.",
        inverse="""    assert_eq!(inverse.len(), 1, "change-page-width inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::ChangePageWidth(step) => {
            assert_eq!(step.id, "page-1", "the inverse must address the same page");
            assert_eq!(step.new_width, 200.0, "the inverse must carry the pre-edit page width");
        }
        other => panic!("change-page-width must invert to change-page-width, got {other:?}"),
    }""",
        probe="""    let delta = produced.diff().pages.as_ref().expect("change-page-width fills the pages delta");
    assert_eq!(delta.patched[0].patch.width, Some(240.0), "change-page-width fills the patch's `width` field");
    assert!(delta.patched[0].patch.height.is_none(), "change-page-width must not emit a `height` patch");""",
    ),
    dict(
        leaf="change-page-height", kind="change-page-height", case="lengthens-page-1",
        mod="tests_lengthens_page_1", transform=t_change_page_height,
        mutation={"ChangePageHeight": {"id": "page-1", "new_height": 360.0}},
        blurb="Proves the page `height` scalar moves independently of `width`.",
        fn1="lengthens_page_1_without_changing_its_width", fn1doc="▶️ `change-page-height` is the vertical twin of `change-page-width` — `width` must stay put.",
        change="""    let page = after.pages.iter().find(|page| page.id == "page-1").expect("page-1 survives");
    assert_eq!(page.height, 360.0, "change-page-height must set the addressed page's height");
    assert_eq!(page.width, 200.0, "change-page-height must leave the width at its BASE value");
    assert_eq!(after.pages[1].height, 300.0, "change-page-height must not resize sibling pages");""",
        fn2="inverse_shortens_page_1_back_to_300", fn2doc="↩️ The inverse is a `change-page-height` carrying BASE's height.",
        inverse="""    assert_eq!(inverse.len(), 1, "change-page-height inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::ChangePageHeight(step) => {
            assert_eq!(step.id, "page-1", "the inverse must address the same page");
            assert_eq!(step.new_height, 300.0, "the inverse must carry the pre-edit page height");
        }
        other => panic!("change-page-height must invert to change-page-height, got {other:?}"),
    }""",
        probe="""    let delta = produced.diff().pages.as_ref().expect("change-page-height fills the pages delta");
    assert_eq!(delta.patched[0].patch.height, Some(360.0), "change-page-height fills the patch's `height` field");
    assert!(delta.patched[0].patch.width.is_none(), "change-page-height must not emit a `width` patch");""",
    ),
    dict(
        leaf="update-page-margins", kind="update-page-margins", case="sets-asymmetric-margins-on-page-1",
        mod="tests_sets_asymmetric_margins_on_page_1", transform=t_update_page_margins,
        mutation={"UpdatePageMargins": {"id": "page-1", "top": 12.0, "right": 18.0, "bottom": 24.0, "left": 6.0}},
        blurb="Proves all four margin edges move in one atomic facet update.",
        fn1="rewrites_all_four_margin_edges_at_once", fn1doc="▶️ `update-page-margins` is an atomic four-field facet: every edge is written, none is inferred.",
        change="""    let margins = &after.pages.iter().find(|page| page.id == "page-1").expect("page-1 survives").margins;
    assert_eq!((margins.top, margins.right, margins.bottom, margins.left), (12.0, 18.0, 24.0, 6.0), "update-page-margins must write all four edges from the payload");
    assert_eq!(after.pages[0].columns.count, 1, "update-page-margins must not touch the column facet");
    assert_eq!(after.pages[1].margins.top, 10.0, "update-page-margins must not touch sibling pages");""",
        fn2="inverse_restores_the_uniform_ten_point_margins", fn2doc="↩️ The inverse carries all four BASE edges, not a partial patch.",
        inverse="""    assert_eq!(inverse.len(), 1, "update-page-margins inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::UpdatePageMargins(step) => {
            assert_eq!(step.id, "page-1", "the inverse must address the same page");
            assert_eq!((step.top, step.right, step.bottom, step.left), (10.0, 10.0, 10.0, 10.0), "the inverse must carry all four pre-edit margin edges");
        }
        other => panic!("update-page-margins must invert to update-page-margins, got {other:?}"),
    }""",
        probe="""    let patch = &produced.diff().pages.as_ref().expect("update-page-margins fills the pages delta").patched[0].patch;
    assert_eq!((patch.margin_top, patch.margin_right, patch.margin_bottom, patch.margin_left), (Some(12.0), Some(18.0), Some(24.0), Some(6.0)), "update-page-margins fills all four margin fields of the patch");
    assert!(patch.columns_count.is_none() && patch.columns_gutter.is_none(), "update-page-margins must not emit a column patch");""",
    ),
    dict(
        leaf="update-page-columns", kind="update-page-columns", case="splits-page-1-into-three-columns",
        mod="tests_splits_page_1_into_three_columns", transform=t_update_page_columns,
        mutation={"UpdatePageColumns": {"id": "page-1", "count": 3, "gutter": 12.0}},
        blurb="Proves count and gutter move together as one atomic facet.",
        fn1="rewrites_count_and_gutter_together", fn1doc="▶️ `update-page-columns` writes the count/gutter pair atomically and leaves the margin facet alone.",
        change="""    let columns = &after.pages.iter().find(|page| page.id == "page-1").expect("page-1 survives").columns;
    assert_eq!((columns.count, columns.gutter), (3, 12.0), "update-page-columns must write both the count and the gutter");
    assert_eq!(after.pages[0].margins.left, 10.0, "update-page-columns must not touch the margin facet");
    assert_eq!(after.pages[1].columns.count, 1, "update-page-columns must not touch sibling pages");""",
        fn2="inverse_restores_the_single_column_grid", fn2doc="↩️ The inverse carries BASE's count AND gutter together.",
        inverse="""    assert_eq!(inverse.len(), 1, "update-page-columns inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::UpdatePageColumns(step) => {
            assert_eq!(step.id, "page-1", "the inverse must address the same page");
            assert_eq!((step.count, step.gutter), (1, 0.0), "the inverse must carry the pre-edit count and gutter");
        }
        other => panic!("update-page-columns must invert to update-page-columns, got {other:?}"),
    }""",
        probe="""    let patch = &produced.diff().pages.as_ref().expect("update-page-columns fills the pages delta").patched[0].patch;
    assert_eq!((patch.columns_count, patch.columns_gutter), (Some(3), Some(12.0)), "update-page-columns fills both column fields of the patch");
    assert!(patch.margin_top.is_none(), "update-page-columns must not emit a margin patch");""",
    ),
    dict(
        leaf="reorder-pages", kind="reorder-pages", case="moves-page-1-behind-page-2",
        mod="tests_moves_page_1_behind_page_2", transform=t_reorder_pages,
        mutation={"ReorderPages": {"id": "page-1", "to_index": 1}},
        blurb="Proves the index-addressed reorder emits a COMPLETE final order, not a swap.",
        fn1="permutes_the_page_order_without_editing_any_page", fn1doc="▶️ `reorder-pages` only permutes — no page record's own fields may change.",
        change="""    assert_eq!(after.pages.iter().map(|page| page.id.as_str()).collect::<Vec<_>>(), vec!["page-2", "page-1"], "reorder-pages must move page-1 to index 1");
    assert_eq!(after.pages[1].name, "Cover", "reorder-pages must not edit the moved page's own fields");
    assert_eq!(after.pages[1].frames.len(), 2, "reorder-pages must not disturb the moved page's frames");""",
        fn2="inverse_reorders_page_1_back_to_the_front", fn2doc="↩️ The inverse is a `reorder-pages` back to the index page-1 occupied in BASE.",
        inverse="""    assert_eq!(inverse.len(), 1, "reorder-pages inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::ReorderPages(step) => {
            assert_eq!(step.id, "page-1", "the inverse must move the same page");
            assert_eq!(step.to_index, 0, "the inverse must target page-1's original index in BASE");
        }
        other => panic!("reorder-pages must invert to reorder-pages, got {other:?}"),
    }""",
        probe="""    let delta = produced.diff().pages.as_ref().expect("reorder-pages fills the pages delta");
    assert_eq!(delta.reordered.as_deref(), Some(["page-2".to_string(), "page-1".to_string()].as_slice()), "reorder-pages emits the complete final id order");
    assert!(delta.added.is_empty() && delta.removed.is_empty() && delta.patched.is_empty(), "reorder-pages touches only the `reordered` arm of the pages delta");""",
    ),
    dict(
        leaf="create-story", kind="create-story", case="appends-story-3",
        mod="tests_appends_story_3", transform=t_create_story,
        mutation={"CreateStory": {"story": STORY_3, "index": 2}},
        blurb="Proves a `TextStory` record enters the stories collection with its style runs.",
        fn1="brings_story_3_into_the_stories_collection", fn1doc="▶️ `create-story` appends the payload's `TextStory` and leaves frames/links alone.",
        change="""    assert_eq!(after.stories.iter().map(|story| story.id.as_str()).collect::<Vec<_>>(), vec!["story-1", "story-2", "story-3"], "create-story appends the new story");
    let created = after.stories.iter().find(|story| story.id == "story-3").expect("create-story inserts story-3");
    assert_eq!(created.content, "Caption.", "create-story must carry the payload story's body");
    assert!(created.style_runs.is_empty(), "create-story must carry the payload story's (empty) style runs");
    assert_eq!(after.links.len(), 2, "create-story must not touch the links collection");""",
        fn2="inverse_deletes_the_story_it_created", fn2doc="↩️ `create-story` always inverts to `delete-story` of the id it minted.",
        inverse="""    assert_eq!(inverse.len(), 1, "create-story inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::DeleteStory(step) => assert_eq!(step.id, "story-3", "the inverse must delete the story id create-story minted"),
        other => panic!("create-story must invert to delete-story, got {other:?}"),
    }""",
        probe="""    let delta = produced.diff().stories.as_ref().expect("create-story fills the stories delta");
    assert_eq!(delta.added.len(), 1, "create-story adds exactly one story");
    assert_eq!(delta.added[0].id, "story-3", "create-story's `added` entry is the payload story");
    assert!(produced.diff().pages.is_none(), "create-story must not emit a pages delta");""",
    ),
    dict(
        leaf="delete-story", kind="delete-story", case="removes-story-2",
        mod="tests_removes_story_2", transform=t_delete_story,
        mutation={"DeleteStory": {"id": "story-2"}},
        blurb="Proves a story leaves the collection without cascading into the text frame that threads it.",
        fn1="drops_story_2_and_leaves_the_text_frame_thread_alone", fn1doc="▶️ `delete-story` removes the record only — there is no cascade into frames' `story_id`.",
        change="""    assert_eq!(after.stories.iter().map(|story| story.id.as_str()).collect::<Vec<_>>(), vec!["story-1"], "delete-story must remove story-2 and only story-2");
    assert_eq!(after.pages[0].frames.len(), 2, "delete-story must not remove the text frame that references a story");
    assert_eq!(after.links.len(), 2, "delete-story must not touch the links collection");""",
        fn2="inverse_recreates_story_2_with_its_body", fn2doc="↩️ The inverse is a `create-story` carrying the removed story's body and its original index.",
        inverse="""    assert_eq!(inverse.len(), 1, "delete-story inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::CreateStory(step) => {
            assert_eq!(step.story.id, "story-2", "the inverse must recreate the removed story");
            assert_eq!(step.story.content, "Spare body.", "the inverse must carry the removed story's body, not a stub");
            assert_eq!(step.index, Some(1), "the inverse must capture the removed story's original index");
        }
        other => panic!("delete-story must invert to create-story, got {other:?}"),
    }""",
        probe="""    let delta = produced.diff().stories.as_ref().expect("delete-story fills the stories delta");
    assert_eq!(delta.removed, vec!["story-2".to_string()], "delete-story's diff carries the id in `removed`");
    assert!(delta.added.is_empty() && delta.patched.is_empty(), "delete-story touches only the `removed` arm of the stories delta");""",
    ),
    dict(
        leaf="edit-story", kind="edit-story", case="rewrites-story-1-body",
        mod="tests_rewrites_story_1_body", transform=t_edit_story,
        mutation={"EditStory": {"id": "story-1", "new_content": "Alpha body, revised."}},
        blurb="Proves the story body is replaced wholesale while style runs survive.",
        fn1="replaces_the_story_body_and_keeps_its_style_runs", fn1doc="▶️ `edit-story` patches only `content`; the `styleRuns` table is not part of the patch.",
        change="""    let story = after.stories.iter().find(|story| story.id == "story-1").expect("story-1 survives");
    assert_eq!(story.content, "Alpha body, revised.", "edit-story must replace the addressed story's body");
    assert!(story.style_runs.is_empty(), "edit-story must leave the style runs exactly as BASE had them");
    assert_eq!(after.stories[1].content, "Spare body.", "edit-story must not rewrite sibling stories");""",
        fn2="inverse_restores_the_original_story_body", fn2doc="↩️ The inverse is an `edit-story` carrying BASE's body text.",
        inverse="""    assert_eq!(inverse.len(), 1, "edit-story inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::EditStory(step) => {
            assert_eq!(step.id, "story-1", "the inverse must address the same story");
            assert_eq!(step.new_content, "Alpha body.", "the inverse must carry the pre-edit body text");
        }
        other => panic!("edit-story must invert to edit-story, got {other:?}"),
    }""",
        probe="""    let delta = produced.diff().stories.as_ref().expect("edit-story fills the stories delta");
    assert_eq!(delta.patched.len(), 1, "edit-story patches exactly one story");
    assert_eq!(delta.patched[0].id, "story-1", "edit-story's patch entry addresses story-1");
    assert_eq!(delta.patched[0].patch.content.as_deref(), Some("Alpha body, revised."), "edit-story fills the patch's `content` field");""",
    ),
    dict(
        leaf="create-link", kind="create-link", case="appends-link-3",
        mod="tests_appends_link_3", transform=t_create_link,
        mutation={"CreateLink": {"link": LINK_3, "index": 2}},
        blurb="Proves a whole `ImageLink` record (path, hash, pixel size, dpi) enters the collection.",
        fn1="brings_link_3_into_the_links_collection", fn1doc="▶️ `create-link` appends the payload's complete `ImageLink`, hash and dpi included.",
        change="""    assert_eq!(after.links.iter().map(|link| link.id.as_str()).collect::<Vec<_>>(), vec!["link-1", "link-2", "link-3"], "create-link appends the new link");
    let created = after.links.iter().find(|link| link.id == "link-3").expect("create-link inserts link-3");
    assert_eq!(created.path, "caption.png", "create-link must carry the payload link's path");
    assert_eq!((created.width, created.height, created.dpi), (200, 150, 144), "create-link must carry the payload link's pixel size and dpi");
    assert_eq!(after.stories.len(), 2, "create-link must not touch the stories collection");""",
        fn2="inverse_deletes_the_link_it_created", fn2doc="↩️ `create-link` always inverts to `delete-link` of the id it minted.",
        inverse="""    assert_eq!(inverse.len(), 1, "create-link inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::DeleteLink(step) => assert_eq!(step.id, "link-3", "the inverse must delete the link id create-link minted"),
        other => panic!("create-link must invert to delete-link, got {other:?}"),
    }""",
        probe="""    let delta = produced.diff().links.as_ref().expect("create-link fills the links delta");
    assert_eq!(delta.added.len(), 1, "create-link adds exactly one link");
    assert_eq!(delta.added[0].hash, "hash-caption", "create-link's `added` entry carries the payload link's hash");
    assert!(produced.diff().stories.is_none(), "create-link must not emit a stories delta");""",
    ),
    dict(
        leaf="delete-link", kind="delete-link", case="removes-link-2",
        mod="tests_removes_link_2", transform=t_delete_link,
        mutation={"DeleteLink": {"id": "link-2"}},
        blurb="Proves a link leaves the collection with no cascade into image frames.",
        fn1="drops_link_2_and_keeps_link_1_intact", fn1doc="▶️ `delete-link` removes the record only — there is no cascade into frames' `link_id`.",
        change="""    assert_eq!(after.links.iter().map(|link| link.id.as_str()).collect::<Vec<_>>(), vec!["link-1"], "delete-link must remove link-2 and only link-2");
    assert_eq!(after.links[0].path, "alpha.png", "delete-link must not rewrite the surviving link's path");
    assert_eq!(after.pages[0].frames.len(), 2, "delete-link does not cascade into the page's frames");""",
        fn2="inverse_recreates_link_2_with_its_hash", fn2doc="↩️ The inverse is a `create-link` carrying the removed link's full record and original index.",
        inverse="""    assert_eq!(inverse.len(), 1, "delete-link inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::CreateLink(step) => {
            assert_eq!(step.link.id, "link-2", "the inverse must recreate the removed link");
            assert_eq!(step.link.hash, "hash-spare", "the inverse must carry the removed link's hash, not a stub");
            assert_eq!(step.index, Some(1), "the inverse must capture the removed link's original index");
        }
        other => panic!("delete-link must invert to create-link, got {other:?}"),
    }""",
        probe="""    let delta = produced.diff().links.as_ref().expect("delete-link fills the links delta");
    assert_eq!(delta.removed, vec!["link-2".to_string()], "delete-link's diff carries the id in `removed`");
    assert!(delta.added.is_empty() && delta.patched.is_empty(), "delete-link touches only the `removed` arm of the links delta");""",
    ),
    dict(
        leaf="change-link-path", kind="change-link-path", case="relinks-link-1-to-a-new-file",
        mod="tests_relinks_link_1_to_a_new_file", transform=t_change_link_path,
        mutation={"ChangeLinkPath": {"id": "link-1", "new_path": "alpha-v2.png"}},
        blurb="Proves relinking rewrites `path` only — hash and pixel size stay stale on purpose.",
        fn1="repoints_the_link_path_but_keeps_the_stale_hash", fn1doc="▶️ `change-link-path` patches `path` alone; `hash`/`width`/`height`/`dpi` are NOT re-derived.",
        change="""    let link = after.links.iter().find(|link| link.id == "link-1").expect("link-1 survives");
    assert_eq!(link.path, "alpha-v2.png", "change-link-path must repoint the addressed link");
    assert_eq!(link.hash, "hash-alpha", "change-link-path must leave the hash untouched — it is not a re-import");
    assert_eq!((link.width, link.height, link.dpi), (800, 600, 300), "change-link-path must leave the pixel size and dpi untouched");
    assert_eq!(after.links[1].path, "spare.png", "change-link-path must not repoint sibling links");""",
        fn2="inverse_repoints_link_1_at_the_original_file", fn2doc="↩️ The inverse is a `change-link-path` carrying BASE's path.",
        inverse="""    assert_eq!(inverse.len(), 1, "change-link-path inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::ChangeLinkPath(step) => {
            assert_eq!(step.id, "link-1", "the inverse must address the same link");
            assert_eq!(step.new_path, "alpha.png", "the inverse must carry the pre-edit path");
        }
        other => panic!("change-link-path must invert to change-link-path, got {other:?}"),
    }""",
        probe="""    let delta = produced.diff().links.as_ref().expect("change-link-path fills the links delta");
    assert_eq!(delta.patched.len(), 1, "change-link-path patches exactly one link");
    assert_eq!(delta.patched[0].patch.path.as_deref(), Some("alpha-v2.png"), "change-link-path fills the patch's `path` field — the only field ImageLinkPatch has");""",
    ),
    dict(
        leaf="create-frame", kind="create-frame", case="inserts-a-rect-frame-at-index-1",
        mod="tests_inserts_a_rect_frame_at_index_1", transform=t_create_frame,
        mutation={"CreateFrame": {"page_id": "page-1", "frame": BADGE, "index": 1, "layer_id": "layer-1"}},
        blurb="Proves the nested frame insert honours `index` AND registers the frame on the named layer.",
        fn1="inserts_at_the_requested_index_and_joins_the_layer", fn1doc="▶️ `create-frame` inserts at the payload index inside the page, but APPENDS the id to the layer's `object_ids`.",
        change="""    let page = &after.pages[0];
    assert_eq!(page.frames.iter().map(|frame| frame.id()).collect::<Vec<_>>(), vec!["frame-rect", "frame-badge", "frame-text"], "create-frame must insert at the payload's index, not append");
    assert_eq!(page.layers[0].object_ids, vec!["frame-rect".to_string(), "frame-text".to_string(), "frame-badge".to_string()], "create-frame appends the new id to the named layer's object list");
    assert_eq!(after.pages[1].frames.len(), 0, "create-frame must not add the frame to any other page");""",
        fn2="inverse_deletes_the_frame_from_the_same_page", fn2doc="↩️ `create-frame` always inverts to `delete-frame` on the same page — it never inspects BASE.",
        inverse="""    assert_eq!(inverse.len(), 1, "create-frame inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::DeleteFrame(step) => {
            assert_eq!(step.page_id, "page-1", "the inverse must address the page create-frame wrote to");
            assert_eq!(step.frame_id, "frame-badge", "the inverse must delete the frame id create-frame minted");
        }
        other => panic!("create-frame must invert to delete-frame, got {other:?}"),
    }""",
        probe="""    let patch = &produced.diff().pages.as_ref().expect("create-frame fills the pages delta").patched[0].patch;
    let added = patch.frame_added.as_ref().expect("create-frame fills the page patch's `frame_added` fragment");
    assert_eq!(added.frame.id(), "frame-badge", "the `frame_added` fragment carries the payload frame verbatim");
    assert_eq!(added.index, Some(1), "the `frame_added` fragment carries the requested insertion index");
    assert_eq!(added.layer_id.as_deref(), Some("layer-1"), "the `frame_added` fragment carries the layer to register on");
    assert!(patch.frame_removed.is_none() && patch.frame_patched.is_none(), "create-frame emits only the `frame_added` fragment");""",
    ),
    dict(
        leaf="delete-frame", kind="delete-frame", case="removes-the-text-frame-and-its-layer-membership",
        mod="tests_removes_the_text_frame_and_its_layer_membership", transform=t_delete_frame,
        mutation={"DeleteFrame": {"page_id": "page-1", "frame_id": "frame-text"}},
        blurb="Proves the frame is dropped AND unregistered from every layer's `object_ids` in one step.",
        fn1="drops_the_frame_and_unregisters_it_from_every_layer", fn1doc="▶️ `delete-frame` cascades into the page's layers — a dangling `object_ids` entry would be a bug.",
        change="""    let page = &after.pages[0];
    assert_eq!(page.frames.iter().map(|frame| frame.id()).collect::<Vec<_>>(), vec!["frame-rect"], "delete-frame must remove the addressed frame and only it");
    assert_eq!(page.layers[0].object_ids, vec!["frame-rect".to_string()], "delete-frame must unregister the id from the page's layers");
    assert_eq!(after.stories.len(), 2, "delete-frame must not cascade into the story the text frame threaded");""",
        fn2="inverse_recreates_the_frame_at_its_index_and_layer", fn2doc="↩️ The inverse is a `create-frame` carrying the removed frame, its index, and the layer that held it.",
        inverse="""    assert_eq!(inverse.len(), 1, "delete-frame inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::CreateFrame(step) => {
            assert_eq!(step.page_id, "page-1", "the inverse must address the same page");
            assert_eq!(step.frame.id(), "frame-text", "the inverse must recreate the removed frame");
            assert_eq!(step.index, Some(1), "the inverse must capture the removed frame's original index within the page");
            assert_eq!(step.layer_id.as_deref(), Some("layer-1"), "the inverse must capture which layer had the frame registered");
        }
        other => panic!("delete-frame must invert to create-frame, got {other:?}"),
    }""",
        probe="""    let patch = &produced.diff().pages.as_ref().expect("delete-frame fills the pages delta").patched[0].patch;
    assert_eq!(patch.frame_removed.as_deref(), Some("frame-text"), "delete-frame fills the page patch's `frame_removed` fragment");
    assert!(patch.frame_added.is_none() && patch.frame_patched.is_none(), "delete-frame emits only the `frame_removed` fragment");""",
    ),
    dict(
        leaf="move-frame", kind="move-frame", case="moves-the-rect-frame",
        mod="tests_moves_the_rect_frame", transform=t_move_frame,
        mutation={"MoveFrame": {"page_id": "page-1", "frame_id": "frame-rect", "new_x": 55.0, "new_y": 65.0}},
        blurb="Proves the bounds origin moves while the extent and rotation stay fixed.",
        fn1="translates_the_bounds_origin_only", fn1doc="▶️ `move-frame` writes `bounds.x`/`bounds.y`; width, height and rotation are untouched.",
        change="""    let page = &after.pages[0];
    let bounds = page.frames.iter().find(|frame| frame.id() == "frame-rect").expect("the rect frame survives").bounds();
    assert_eq!((bounds.x, bounds.y), (55.0, 65.0), "move-frame must write the payload position into the frame bounds");
    assert_eq!((bounds.width, bounds.height), (60.0, 40.0), "move-frame must not resize the frame");
    assert_eq!(bounds.rotation, 0.0, "move-frame must not rotate the frame");
    assert_eq!(page.frames.iter().find(|frame| frame.id() == "frame-text").expect("the text frame survives").bounds().x, 20.0, "move-frame must not move sibling frames");""",
        fn2="inverse_moves_the_rect_frame_back", fn2doc="↩️ The inverse is a `move-frame` carrying the bounds origin captured from BASE.",
        inverse="""    assert_eq!(inverse.len(), 1, "move-frame inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::MoveFrame(step) => {
            assert_eq!((step.page_id.as_str(), step.frame_id.as_str()), ("page-1", "frame-rect"), "the inverse must address the same frame on the same page");
            assert_eq!((step.new_x, step.new_y), (20.0, 30.0), "the inverse must carry the pre-move bounds origin");
        }
        other => panic!("move-frame must invert to move-frame, got {other:?}"),
    }""",
        probe="""    let patch = &produced.diff().pages.as_ref().expect("move-frame fills the pages delta").patched[0].patch;
    let patched = patch.frame_patched.as_ref().expect("move-frame fills the page patch's `frame_patched` fragment");
    assert_eq!(patched.frame_id, "frame-rect", "the `frame_patched` fragment addresses the moved frame");
    assert_eq!((patched.patch.x, patched.patch.y), (Some(55.0), Some(65.0)), "move-frame fills only x/y of the frame patch");
    assert!(patched.patch.width.is_none() && patched.patch.height.is_none(), "move-frame must not emit a size patch");""",
    ),
    dict(
        leaf="resize-frame", kind="resize-frame", case="resizes-the-rect-frame",
        mod="tests_resizes_the_rect_frame", transform=t_resize_frame,
        mutation={"ResizeFrame": {"page_id": "page-1", "frame_id": "frame-rect", "new_width": 90.0, "new_height": 70.0}},
        blurb="Proves the bounds extent changes while the origin stays anchored.",
        fn1="rescales_the_bounds_extent_only", fn1doc="▶️ `resize-frame` writes `bounds.w`/`bounds.h`; the origin stays anchored (no re-centering).",
        change="""    let page = &after.pages[0];
    let bounds = page.frames.iter().find(|frame| frame.id() == "frame-rect").expect("the rect frame survives").bounds();
    assert_eq!((bounds.width, bounds.height), (90.0, 70.0), "resize-frame must write the payload extent into the frame bounds");
    assert_eq!((bounds.x, bounds.y), (20.0, 30.0), "resize-frame must keep the origin anchored — it does not re-centre the frame");
    assert_eq!(page.frames.iter().find(|frame| frame.id() == "frame-text").expect("the text frame survives").bounds().width, 160.0, "resize-frame must not resize sibling frames");""",
        fn2="inverse_restores_the_original_extent", fn2doc="↩️ The inverse is a `resize-frame` carrying the extent captured from BASE.",
        inverse="""    assert_eq!(inverse.len(), 1, "resize-frame inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::ResizeFrame(step) => {
            assert_eq!((step.page_id.as_str(), step.frame_id.as_str()), ("page-1", "frame-rect"), "the inverse must address the same frame on the same page");
            assert_eq!((step.new_width, step.new_height), (60.0, 40.0), "the inverse must carry the pre-resize extent");
        }
        other => panic!("resize-frame must invert to resize-frame, got {other:?}"),
    }""",
        probe="""    let patch = &produced.diff().pages.as_ref().expect("resize-frame fills the pages delta").patched[0].patch;
    let patched = patch.frame_patched.as_ref().expect("resize-frame fills the page patch's `frame_patched` fragment");
    assert_eq!((patched.patch.width, patched.patch.height), (Some(90.0), Some(70.0)), "resize-frame fills only width/height of the frame patch");
    assert!(patched.patch.x.is_none() && patched.patch.y.is_none(), "resize-frame must not emit a position patch");""",
    ),
    dict(
        leaf="change-frame-fill", kind="change-frame-fill", case="repaints-the-rect-frame-fill",
        mod="tests_repaints_the_rect_frame_fill", transform=t_change_frame_fill,
        mutation={"ChangeFrameFill": {"page_id": "page-1", "frame_id": "frame-rect", "new_fill": [0.5, 0.25, 0.75, 1.0]}},
        blurb="Proves the Rect-only `fill` field is replaced and `stroke` is left alone.",
        fn1="repaints_the_rect_fill_without_touching_the_stroke", fn1doc="▶️ `change-frame-fill` is a Rect-variant-specific field patch; `stroke` is a different mutation's business.",
        change="""    let page = &after.pages[0];
    let frame = page.frames.iter().find(|frame| frame.id() == "frame-rect").expect("the rect frame survives");
    let crate::artifacts::layout::Frame::Rect { fill, stroke, bounds, .. } = frame else { panic!("change-frame-fill targets the rect frame") };
    assert_eq!(*fill, Some([0.5, 0.25, 0.75, 1.0]), "change-frame-fill must write the payload RGBA into the rect's fill");
    assert_eq!(*stroke, None, "change-frame-fill must leave the stroke cleared as BASE had it");
    assert_eq!((bounds.x, bounds.width), (20.0, 60.0), "change-frame-fill must not move or resize the frame");""",
        fn2="inverse_restores_the_white_fill", fn2doc="↩️ The inverse is a `change-frame-fill` carrying the RGBA captured from BASE's rect.",
        inverse="""    assert_eq!(inverse.len(), 1, "change-frame-fill inverts to exactly one step on a rect frame");
    match &inverse[0] {
        LayoutMutation::ChangeFrameFill(step) => {
            assert_eq!((step.page_id.as_str(), step.frame_id.as_str()), ("page-1", "frame-rect"), "the inverse must address the same frame on the same page");
            assert_eq!(step.new_fill, Some([1.0, 1.0, 1.0, 1.0]), "the inverse must carry the pre-edit fill");
        }
        other => panic!("change-frame-fill must invert to change-frame-fill, got {other:?}"),
    }""",
        probe="""    let patch = &produced.diff().pages.as_ref().expect("change-frame-fill fills the pages delta").patched[0].patch;
    let patched = patch.frame_patched.as_ref().expect("change-frame-fill fills the page patch's `frame_patched` fragment");
    assert_eq!(patched.patch.fill, Some(Some([0.5, 0.25, 0.75, 1.0])), "change-frame-fill fills the doubly-optional `fill` field (outer Some = changed, inner Some = now painted)");
    assert!(patched.patch.stroke.is_none(), "change-frame-fill must leave the `stroke` field of the frame patch unset");""",
    ),
    dict(
        leaf="change-frame-stroke", kind="change-frame-stroke", case="adds-a-stroke-to-the-rect-frame",
        mod="tests_adds_a_stroke_to_the_rect_frame", transform=t_change_frame_stroke,
        mutation={"ChangeFrameStroke": {"page_id": "page-1", "frame_id": "frame-rect", "new_stroke": [0.0, 0.0, 0.0, 1.0]}},
        blurb="Proves the Rect-only `stroke` field goes from cleared to painted and `fill` is left alone.",
        fn1="paints_a_stroke_without_touching_the_fill", fn1doc="▶️ `change-frame-stroke` writes the Rect-variant `stroke`; the `fill` stays as BASE painted it.",
        change="""    let page = &after.pages[0];
    let frame = page.frames.iter().find(|frame| frame.id() == "frame-rect").expect("the rect frame survives");
    let crate::artifacts::layout::Frame::Rect { fill, stroke, .. } = frame else { panic!("change-frame-stroke targets the rect frame") };
    assert_eq!(*stroke, Some([0.0, 0.0, 0.0, 1.0]), "change-frame-stroke must write the payload RGBA into the rect's stroke");
    assert_eq!(*fill, Some([1.0, 1.0, 1.0, 1.0]), "change-frame-stroke must leave the fill exactly as BASE painted it");""",
        fn2="inverse_clears_the_stroke_again", fn2doc="↩️ BASE had no stroke, so the inverse carries `None` — the doubly-optional patch's \"cleared\" arm.",
        inverse="""    assert_eq!(inverse.len(), 1, "change-frame-stroke inverts to exactly one step on a rect frame");
    match &inverse[0] {
        LayoutMutation::ChangeFrameStroke(step) => {
            assert_eq!((step.page_id.as_str(), step.frame_id.as_str()), ("page-1", "frame-rect"), "the inverse must address the same frame on the same page");
            assert!(step.new_stroke.is_none(), "the inverse must carry BASE's cleared stroke");
        }
        other => panic!("change-frame-stroke must invert to change-frame-stroke, got {other:?}"),
    }""",
        probe="""    let patch = &produced.diff().pages.as_ref().expect("change-frame-stroke fills the pages delta").patched[0].patch;
    let patched = patch.frame_patched.as_ref().expect("change-frame-stroke fills the page patch's `frame_patched` fragment");
    assert_eq!(patched.patch.stroke, Some(Some([0.0, 0.0, 0.0, 1.0])), "change-frame-stroke fills the doubly-optional `stroke` field");
    assert!(patched.patch.fill.is_none(), "change-frame-stroke must leave the `fill` field of the frame patch unset");""",
    ),
    dict(
        leaf="change-frame-wrap-mode", kind="change-frame-wrap-mode", case="switches-the-text-frame-to-column-wrap",
        mod="tests_switches_the_text_frame_to_column_wrap", transform=t_change_frame_wrap_mode,
        mutation={"ChangeFrameWrapMode": {"page_id": "page-1", "frame_id": "frame-text", "new_wrap_mode": "column"}},
        blurb="Proves the Text-only `wrap_mode` string is replaced and the column count is left alone.",
        fn1="switches_the_wrap_mode_without_recolumning", fn1doc="▶️ `change-frame-wrap-mode` is a Text-variant-specific field patch; `columns` belongs to a different mutation.",
        change="""    let page = &after.pages[0];
    let frame = page.frames.iter().find(|frame| frame.id() == "frame-text").expect("the text frame survives");
    let crate::artifacts::layout::Frame::Text { wrap_mode, columns, story_id, .. } = frame else { panic!("change-frame-wrap-mode targets the text frame") };
    assert_eq!(wrap_mode, "column", "change-frame-wrap-mode must write the payload wrap mode");
    assert_eq!(*columns, 1, "change-frame-wrap-mode must leave the column count at its BASE value");
    assert_eq!(story_id, "story-1", "change-frame-wrap-mode must not rethread the frame's story");""",
        fn2="inverse_restores_the_box_wrap_mode", fn2doc="↩️ The inverse is a `change-frame-wrap-mode` carrying BASE's wrap mode string.",
        inverse="""    assert_eq!(inverse.len(), 1, "change-frame-wrap-mode inverts to exactly one step on a text frame");
    match &inverse[0] {
        LayoutMutation::ChangeFrameWrapMode(step) => {
            assert_eq!((step.page_id.as_str(), step.frame_id.as_str()), ("page-1", "frame-text"), "the inverse must address the same frame on the same page");
            assert_eq!(step.new_wrap_mode, "box", "the inverse must carry the pre-edit wrap mode");
        }
        other => panic!("change-frame-wrap-mode must invert to change-frame-wrap-mode, got {other:?}"),
    }""",
        probe="""    let patch = &produced.diff().pages.as_ref().expect("change-frame-wrap-mode fills the pages delta").patched[0].patch;
    let patched = patch.frame_patched.as_ref().expect("change-frame-wrap-mode fills the page patch's `frame_patched` fragment");
    assert_eq!(patched.patch.wrap_mode.as_deref(), Some("column"), "change-frame-wrap-mode fills the `wrap_mode` field of the frame patch");
    assert!(patched.patch.columns.is_none(), "change-frame-wrap-mode must leave the `columns` field of the frame patch unset");""",
    ),
    dict(
        leaf="change-frame-columns", kind="change-frame-columns", case="splits-the-text-frame-into-two-columns",
        mod="tests_splits_the_text_frame_into_two_columns", transform=t_change_frame_columns,
        mutation={"ChangeFrameColumns": {"page_id": "page-1", "frame_id": "frame-text", "new_columns": 2}},
        blurb="Proves the Text-only `columns` count is replaced and the wrap mode is left alone.",
        fn1="recolumns_the_text_frame_without_changing_its_wrap_mode", fn1doc="▶️ `change-frame-columns` writes the Text-variant `columns` count only.",
        change="""    let page = &after.pages[0];
    let frame = page.frames.iter().find(|frame| frame.id() == "frame-text").expect("the text frame survives");
    let crate::artifacts::layout::Frame::Text { wrap_mode, columns, inset, .. } = frame else { panic!("change-frame-columns targets the text frame") };
    assert_eq!(*columns, 2, "change-frame-columns must write the payload column count");
    assert_eq!(wrap_mode, "box", "change-frame-columns must leave the wrap mode at its BASE value");
    assert_eq!(inset.width, 0.0, "change-frame-columns must not touch the text inset");""",
        fn2="inverse_restores_the_single_column_text_frame", fn2doc="↩️ The inverse is a `change-frame-columns` carrying BASE's column count.",
        inverse="""    assert_eq!(inverse.len(), 1, "change-frame-columns inverts to exactly one step on a text frame");
    match &inverse[0] {
        LayoutMutation::ChangeFrameColumns(step) => {
            assert_eq!((step.page_id.as_str(), step.frame_id.as_str()), ("page-1", "frame-text"), "the inverse must address the same frame on the same page");
            assert_eq!(step.new_columns, 1, "the inverse must carry the pre-edit column count");
        }
        other => panic!("change-frame-columns must invert to change-frame-columns, got {other:?}"),
    }""",
        probe="""    let patch = &produced.diff().pages.as_ref().expect("change-frame-columns fills the pages delta").patched[0].patch;
    let patched = patch.frame_patched.as_ref().expect("change-frame-columns fills the page patch's `frame_patched` fragment");
    assert_eq!(patched.patch.columns, Some(2), "change-frame-columns fills the `columns` field of the frame patch");
    assert!(patched.patch.wrap_mode.is_none(), "change-frame-columns must leave the `wrap_mode` field of the frame patch unset");""",
    ),
]

TEMPLATE = '''//! \U0001f9ea️ `{kind}` fixture — `{case}`.
//!
//! {blurb}
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::LayoutSnapshot;
use protocol::{{Mutation, MutationDiff}};

const BEFORE: &str = include_str!("\U0001f4f8️snapshot/⬅️before/\U0001f523️component.json");
const AFTER: &str = include_str!("\U0001f4f8️snapshot/➡️after/\U0001f523️component.json");
const MUTATION: &str = include_str!("\U0001f9a0️mutation/\U0001f523️component.json");
const OUTCOME: &str = include_str!("\U0001f3af️outcome/\U0001f523️component.json");

fn before() -> LayoutSnapshot {{
    serde_json::from_str(BEFORE).expect("{kind}/{case}: before snapshot decodes")
}}
fn expected_after() -> LayoutSnapshot {{
    serde_json::from_str(AFTER).expect("{kind}/{case}: after snapshot decodes")
}}
fn mutation() -> LayoutMutation {{
    serde_json::from_str(MUTATION).expect("{kind}/{case}: mutation decodes")
}}
fn applied() -> LayoutSnapshot {{
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("{kind} applies to its committed before-snapshot")
}}

/// {fn1doc}
#[semio_framework_async_macros::async_test]
async fn {fn1}() {{
    let after = applied();
{change}
    assert_eq!(after, expected_after(), "{kind}/{case}: applied state differs from the committed after-snapshot");
}}

/// {fn2doc}
#[semio_framework_async_macros::async_test]
async fn {fn2}() {{
    let base = before();
    let inverse = mutation().inverse(&base);
{inverse}
    let mut snapshot = applied();
    for step in &inverse {{
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("{kind}/{case}: inverse step applies");
    }}
    assert_eq!(snapshot, base, "{kind}/{case}: inverse did not restore the before-snapshot");
}}

/// \U0001f523️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {{
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {{
        let decoded: LayoutSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "{kind}/{case}: committed {{label}} JSON is not canonical");
    }}
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "{kind}/{case}: committed mutation JSON is not canonical");
}}

/// \U0001f3af️ The declared outcome matches what `{kind}`'s own diff builder actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {{
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "{kind}/{case}: this fixture declares an applied outcome");
    let base = before();
    let produced = mutation().diff(&base);
    assert!(produced.messages().is_empty(), "{kind}/{case}: declared clean-applied but the diff builder reported {{:?}}", produced.messages());
{probe}
}}
'''

def dump(path, obj):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(obj, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")

LEAVES = {}
for entry in sorted(os.listdir(MUT)):
    if not (MUT / entry).is_dir():
        continue
    slug = re.sub(r"^[^a-z]*", "", entry)
    LEAVES[slug] = entry

written = []
for spec in CASES:
    leaf = MUT / LEAVES[spec["leaf"]]
    assert leaf.is_dir(), f"missing leaf {spec['leaf']}"
    case_dir = leaf / "\U0001f9ea️tests" / spec["case"]
    before_doc = base()
    after_doc = base()
    spec["transform"](after_doc)
    dump(case_dir / "\U0001f4f8️snapshot/⬅️before/\U0001f523️component.json", before_doc)
    dump(case_dir / "\U0001f4f8️snapshot/➡️after/\U0001f523️component.json", after_doc)
    dump(case_dir / "\U0001f9a0️mutation/\U0001f523️component.json", spec["mutation"])
    dump(case_dir / "\U0001f3af️outcome/\U0001f523️component.json", APPLIED)
    rs = TEMPLATE.format(**spec)
    (case_dir / "\U0001f980️component.rs").write_text(rs, encoding="utf-8")
    written.append((LEAVES[spec["leaf"]], spec["case"], spec["mod"]))

for row in written:
    print(row[0], row[1], row[2], sep="\t")
print(len(written), "layout cases")
