import { readFileSync, writeFileSync } from "fs";

const p = process.argv[2];
let s = readFileSync(p, "utf8");

s = s.replaceAll(
  `            let VcsDocumentApp { app, cache, .. } = self;
            let (_, projection, config, history) = cache.as_ref().expect("cache refreshed above");
            let doc = DocumentView { projection, history };
            let cfg = ConfigView { projection: config };
            let draft_projection = self.draft_store.projection().expect("draft projection");
            let draft = DraftView { projection: &draft_projection };
            A::window_engagements(&doc, &cfg, &draft)`,
  `            let (_, projection, config, history) = self.cache.as_ref().expect("cache refreshed above");
            let doc = DocumentView { projection, history };
            let cfg = ConfigView { projection: config };
            A::window_engagements(&doc, &cfg)`,
);

s = s.replaceAll(
  `            let VcsDocumentApp { app, cache, .. } = self;
            let (_, projection, config, history) = cache.as_ref().expect("cache refreshed above");
            let doc = DocumentView { projection, history };
            let cfg = ConfigView { projection: config };
            let draft_projection = self.draft_store.projection().expect("draft projection");
            let draft = DraftView { projection: &draft_projection };
            A::window_measures(&doc, &cfg, &draft)`,
  `            let (_, projection, config, history) = self.cache.as_ref().expect("cache refreshed above");
            let doc = DocumentView { projection, history };
            let cfg = ConfigView { projection: config };
            A::window_measures(&doc, &cfg)`,
);

s = s.replaceAll(
  "fn copy_fragment(&self, doc: &DocumentView<'_, TestProjection>, _cfg: &ConfigView<'_, TestConfig>)",
  "fn copy_fragment(doc: &DocumentView<'_, TestProjection>, _cfg: &ConfigView<'_, TestConfig>)",
);
s = s.replaceAll(
  "fn context_menu(&self, _request: &ContextMenuRequest, doc: &DocumentView<'_, TestProjection>, _cfg: &ConfigView<'_, TestConfig>, registry: &AppActionRegistry)",
  "fn context_menu(_request: &ContextMenuRequest, doc: &DocumentView<'_, TestProjection>, _cfg: &ConfigView<'_, TestConfig>, registry: &AppActionRegistry)",
);

writeFileSync(p, s);
console.log("fixed window and testapp");
