3256:        // 🕶️ View is ephemeral cursor/selection/camera state; Shell is an outside-the-document
3257:        // host effect — distinct icons so a folded ×count row reads at a glance.
3258:        ActionKind::View => IconName::Eye,
3259:        ActionKind::Shell => IconName::Monitor,
3260:    }
3261:}
3262:
3263:/// @emoji 🕰️ Builds the framework's history panel body from a `HistoryView` as a pure side-panel
3264:/// `Tree` (same shape as Document/Catalogue): an Actions section (undo/redo/checkpoint/alternative +
3265:/// filter control) and a Commands section of newest-first rows with optional "backwards" revert.
3266:/// Shared by both renderers — `VcsDocumentApp::render` returns this verbatim for
3267:/// `FRAMEWORK_HISTORY_BODY_KEY`.
3268:pub fn ui_history_panel(history: &HistoryView, controller_id: &stpub fn ui_history_panel(history: &HistoryView, controller_id: &str, is_de: bool) -> UiNode {
3269:    let act = |action: &str, args: Option<DslValue>| ActionDescriptor { controller_id: controller_id.to_string(), action: action.to_string(), args };
3270:    let action_item = |id: &str, icon_id: IconName, label_en: &str, label_de: &str, action: &str, enabled: bool| {
3271:        let mut item = UiTreeItemNode::base(id, if is_de { label_de } else { label_en });
3272:        item.icon_id = Some(icon_id);
3273:        item.action = Some(act(action, None));
3274:        if !enabled {
3275:            item.presence = UiPresence::state(UiState::Disabled);
3276:        }
3277:        item
3278:    };
3279:
3280:    let filter_value = match history.command_filter {
3281:        HistoryCommandFilter::All => "all",
3282:        HistoryCommandFilter::WithoutOperations => "withoutOperations",
3283:        HistoryCommandFilter::OnlyOperations => "onlyOperations",
3284:    };
3285:    let mut filter_item = UiTreeItemNode::base("framework.history.filter", if is_de { "Filter" } else { "Filter" });
3286:    filter_item.icon_id = Some(IconName::Filter);
3287:    filter_item.control = Some(UiControlNode::Select(UiSelectNode {
3288:        id: "framework.history.filter.control".into(),
3289:        value: filter_value.into(),
3290:        items: vec![
3291:            UiSelectItem { value: "all".into(), label: if is_de { "Alle" } else { "All" }.into() },
3292:            UiSelectItem { value: "withoutOperations".into(), label: if is_de { "Ohne Operationen" } else { "Without Operations" }.into() },
3293:            UiSelectItem { value: "onlyOperations".into(), label: if is_de { "Nur Operationen" } else { "Only Operations" }.into() },
3294:        ],
3295:        placeholder: None,
3296:        on_change: act(SET_HISTORY_COMMAND_FILTER_ACTION_ID, None),
3297:        presence: UiPresence::default(),
3298:        menu: None,
3299:    }));
3300:
3301:    let command_items: Vec<UiTreeItemNode> = history
3302:        .commands
3303:        .iter()
3304:        .filter(|entry| match history.command_filter {
3305:            HistoryCommandFilter::All => true,
3306:            HistoryCommandFilter::WithoutOperations => entry.edit_id.is_none(),
3307:            HistoryCommandFilter::OnlyOperations => entry.edit_id.is_some(),
3308:        })
3309:        .take(HISTORY_PANEL_ROW_LIMIT)
3310:        .map(|entry| {
3311:            // 🔢️ A folded row (`count > 1`) shows "Label xN" instead of the bare label.
3312:            let label = if entry.count > 1 { format!("{} x{}", entry.label, entry.count) } else { entry.label.clone() };
3313:            let mut item = UiTreeItemNode::base(format!("framework.history.entry.{}", entry.seq), label);
3314:            item.description = if entry.op_lines.is_empty() { None } else { Some(entry.op_lines.join(" · ")) };
3315:            item.icon_id = Some(history_panel_icon_id(entry.kind));
3316:            item.dimmed = (entry.edit_id.is_some() && !entry.applied).then_some(true);
3317:            if entry.revertible {
3318:                item.actions = Some(vec![UiTreeItemAction {
3319:                    icon_id: IconName::RotateCcw,
3320:                    label: Some(if is_de { "Zurück bis hier" } else { "Backwards" }.into()),
3321:                    action: act(REVERT_TO_COMMAND_ACTION_ID, Some(DslValue::Object(vec![("entrySeq".into(), DslValue::Number(entry.seq as f64))]))),
3322:                    reveal_on_hover: Some(true),
3323:                }]);
3324:            }
3325:            item
3326:        })
3327:        .collect();
3328:
3329:    UiNode::Tree(UiTreeNode {
3330:        sections: vec![
3331:            UiTreeSectionNode {
3332:                id: "framework.history.actions".into(),
3333:                label: Some(if is_de { "Aktionen" } else { "Actions" }.into()),
3334:                default_open: Some(true),
3335:                presence: UiPresence::default(),
3336:                items: vec![
3337:                    action_item("framework.history.undo", IconName::Undo, "Undo", "Rückgängig", "undo", history.can_undo),
3338:                    action_item("framework.history.redo", IconName::Redo, "Redo", "Wiederholen", "redo", history.can_redo),
3339:                    action_item("framework.history.commitCheckpoint", IconName::GitCommit, "Commit Checkpoint", "Checkpoint", "commitCheckpoint", true),
3340:                    action_item("framework.history.createAlternative", IconName::GitBranch, "Create Alternative", "Alternative erstellen", "createAlternative", true),
3341:                    filter_item,
3342:                ],
3343:            },
3344:            UiTreeSectionNode {
3345:                id: "framework.history.commands".into(),
3346:                label: Some(if is_de { "Befehle" } else { "Commands" }.into()),
3347:                default_open: Some(true),
3348:                presence: UiPresence::default(),
3349:                items: command_items,
3350:            },
3351:        ],
3352:        presence: UiPresence::default(),
3353:        selected_ids: None,
3354:        highlighted_ids: None,
3355:        selection_change: None,
3356:        drop_action: None,
3357:        menu: None,
3358:    })
3359:}App::handle` emits: zero-or-more typed document operations (applied
3360:/// through the document store with a true inverse) and zero-or-more typed config operations (applied
3361:/// through the config store, also with a true inverse via `ConfigOperation::backwards` — the config-op
3362:/// twin of a document op, replacing the old `ActionEmit::inverse`/`InverseAction` ad hoc self-computed
3363:/// inverse: a former "View"-kind action now just emits a `ConfigOperation` and gets a real backwards
3364:/// for free), plus an optional description/coalesce key for the resulting edit(s), host effects
3365:/// (navigate/export/spawn…), and app events. `B1` rename: was `ActionEmit`, `operations` renamed
3366:/// `document_operations` to sit next to `config_operations` unambiguously.
3367:pub struct Emit<Operation, ConfigOperation = NoConfigOperation> {
3368:    pub document_operations: Vec<Operation>,
3369:    pub config_operations: Vec<ConfigOperation>,
3370:    pub description: Option<String>,
3371:    pub coalesce_key: Option<String>,
3372:    pub effects: Vec<HostEffect>,
3373:    pub events: Vec<AppEvent>,
3374:    /// 🐢️ Which rendered UI sections this action actually invalidates — `Full` (the default) preserves
3375:    /// today's whole-shell-refresh behavior for every app that doesn't opt in to narrower scopes.
3376:    pub ui_scope: semio_framework_core::kernel::UiDirtyScope,
3377:}
3378:
3379:impl<Operation, ConfigOperation> Default for Emit<Operation, ConfigOperation> {
3380:    fn default() -> Self {
3381:        Self {
3382:            document_operations: Vec::new(),
3383:            config_operations: Vec::new(),
3384:            description: None,
3385:            coalesce_key: None,
3386:            effects: Vec::new(),
3387:            events: Vec::new(),
3388:            ui_scope: semio_framework_core::kernel::UiDirtyScope::default(),
3389:        }
3390:    }
3391:}
3392:
3393:impl<Operation, ConfigOperation> Emit<Operation, ConfigOperation> {
3394:    /// @emoji ✏️ A document-operation emission carrying `document_operations` and nothing else.
3395:    pub fn operations(document_operations: Vec<Operation>) -> Self {
3396:        Self { document_operations, ..Default::default() }
3397:    }
3398:
3399:    /// @emoji 🔁️ Preview pattern (a): a per-tick coalesced DOCUMENT emission. The `coalesce_key` folds
3400:    /// every tick of one live gesture (drag/scrub) into a single amendable edit, so the whole gesture is