from pathlib import Path

p = Path(r"c:\git\semio\semio\client\lib\rs\lib.rs")
t = p.read_text(encoding="utf-8")

pairs = [
    (
        """        #[graphql(name = "changeDescription")]
        async fn change_description(&self, ctx: &Context<'_>, #[graphql(name = "newDescription")] new_description: String) -> async_graphql::Result<crate::operation::ResponseInterface> {
            let _ = (ctx, self, new_description);
            Ok(crate::operation::CommandResponse::fail_msg("not implemented").await.into())
        }

        #[graphql(name = "createTag")]""",
        """        #[graphql(name = "changeDescription")]
        async fn change_description(&self, ctx: &Context<'_>, #[graphql(name = "newDescription")] new_description: String) -> async_graphql::Result<crate::operation::ResponseInterface> {
            let rt = ctx.data::<Arc<ParentStore>>()?;
            let kit = rt.wip_graph.materialized_head_kit_from_ref().await;
            let entity_id = kit.workspace_kit_id().await;
            Ok(dispatch_unsaved_kit_operation(
                rt,
                &self.change_id,
                crate::operation::Operation::ChangeDescription { scope: Scope::Entity { entity_id }, input: Input::Description { description: Some(new_description) } },
            )
            .await
            .into())
        }

        #[graphql(name = "createTag")]""",
    ),
    (
        """        #[graphql(name = "deleteConcept")]
        async fn delete_concept(&self, ctx: &Context<'_>, id: Id) -> async_graphql::Result<crate::operation::ResponseInterface> {
            let _ = (ctx, self, id);
            Ok(crate::operation::CommandResponse::fail_msg("not implemented").await.into())
        }

        #[graphql(name = "deleteConcepts")]
        async fn delete_concepts(&self, ctx: &Context<'_>, ids: Vec<Id>) -> async_graphql::Result<crate::operation::ResponseInterface> {
            let _ = (ctx, self, ids);
            Ok(crate::operation::CommandResponse::fail_msg("not implemented").await.into())
        }

        #[graphql(name = "createQuality")]""",
        """        #[graphql(name = "deleteConcept")]
        async fn delete_concept(&self, ctx: &Context<'_>, id: Id) -> async_graphql::Result<crate::operation::ResponseInterface> {
            let rt = ctx.data::<Arc<ParentStore>>()?;
            Ok(dispatch_unsaved_kit_operation(rt, &self.change_id, crate::operation::Operation::DeleteConcept { scope: Scope::Concept { concept_id: id }, input: Input::None }).await.into())
        }

        #[graphql(name = "deleteConcepts")]
        async fn delete_concepts(&self, ctx: &Context<'_>, ids: Vec<Id>) -> async_graphql::Result<crate::operation::ResponseInterface> {
            let rt = ctx.data::<Arc<ParentStore>>()?;
            let mut last = crate::operation::CommandResponse::fail_msg("no concept ids").await;
            for id in ids {
                last = dispatch_unsaved_kit_operation(rt, &self.change_id, crate::operation::Operation::DeleteConcept { scope: Scope::Concept { concept_id: id }, input: Input::None }).await;
                if !last.ok {
                    break;
                }
            }
            Ok(last.into())
        }

        #[graphql(name = "createQuality")]""",
    ),
    (
        """        #[graphql(name = "deleteQuality")]
        async fn delete_quality(&self, ctx: &Context<'_>, id: Id) -> async_graphql::Result<crate::operation::ResponseInterface> {
            let _ = (ctx, self, id);
            Ok(crate::operation::CommandResponse::fail_msg("not implemented").await.into())
        }

        #[graphql(name = "deleteQualities")]
        async fn delete_qualities(&self, ctx: &Context<'_>, ids: Vec<Id>) -> async_graphql::Result<crate::operation::ResponseInterface> {
            let _ = (ctx, self, ids);
            Ok(crate::operation::CommandResponse::fail_msg("not implemented").await.into())
        }

        #[graphql(name = "createType")]""",
        """        #[graphql(name = "deleteQuality")]
        async fn delete_quality(&self, ctx: &Context<'_>, id: Id) -> async_graphql::Result<crate::operation::ResponseInterface> {
            let rt = ctx.data::<Arc<ParentStore>>()?;
            Ok(dispatch_unsaved_kit_operation(rt, &self.change_id, crate::operation::Operation::DeleteQuality { scope: Scope::Quality { quality_id: id }, input: Input::None }).await.into())
        }

        #[graphql(name = "deleteQualities")]
        async fn delete_qualities(&self, ctx: &Context<'_>, ids: Vec<Id>) -> async_graphql::Result<crate::operation::ResponseInterface> {
            let rt = ctx.data::<Arc<ParentStore>>()?;
            let mut last = crate::operation::CommandResponse::fail_msg("no quality ids").await;
            for id in ids {
                last = dispatch_unsaved_kit_operation(rt, &self.change_id, crate::operation::Operation::DeleteQuality { scope: Scope::Quality { quality_id: id }, input: Input::None }).await;
                if !last.ok {
                    break;
                }
            }
            Ok(last.into())
        }

        #[graphql(name = "createType")]""",
    ),
    (
        """    impl TagOperationInput {
        #[graphql(name = "rename")]
        async fn rename(&self, ctx: &Context<'_>, #[graphql(name = "newName")] new_name: String) -> async_graphql::Result<crate::operation::ResponseInterface> {
            let _ = (ctx, self, new_name);
            Ok(crate::operation::CommandResponse::fail_msg("not implemented").await.into())
        }
        #[graphql(name = "changeDescription")]
        async fn change_description(&self, ctx: &Context<'_>, #[graphql(name = "newDescription")] new_description: String) -> async_graphql::Result<crate::operation::ResponseInterface> {
            let _ = (ctx, self, new_description);
            Ok(crate::operation::CommandResponse::fail_msg("not implemented").await.into())
        }""",
        """    impl TagOperationInput {
        #[graphql(name = "rename")]
        async fn rename(&self, ctx: &Context<'_>, #[graphql(name = "newName")] new_name: String) -> async_graphql::Result<crate::operation::ResponseInterface> {
            let rt = ctx.data::<Arc<ParentStore>>()?;
            Ok(dispatch_unsaved_kit_operation(
                rt,
                &self.change_id,
                crate::operation::Operation::RenameTag { scope: Scope::Tag { tag_id: self.tag_id.clone() }, input: Input::Name { name: new_name } },
            )
            .await
            .into())
        }
        #[graphql(name = "changeDescription")]
        async fn change_description(&self, ctx: &Context<'_>, #[graphql(name = "newDescription")] new_description: String) -> async_graphql::Result<crate::operation::ResponseInterface> {
            let rt = ctx.data::<Arc<ParentStore>>()?;
            Ok(dispatch_unsaved_kit_operation(
                rt,
                &self.change_id,
                crate::operation::Operation::ChangeDescription {
                    scope: Scope::Entity { entity_id: self.tag_id.clone() },
                    input: Input::Description { description: Some(new_description) },
                },
            )
            .await
            .into())
        }""",
    ),
    (
        """        #[graphql(name = "changeDescription")]
        async fn change_description(&self, ctx: &Context<'_>, #[graphql(name = "newDescription")] new_description: String) -> async_graphql::Result<crate::operation::ResponseInterface> {
            let _ = (ctx, self, new_description);
            Ok(crate::operation::CommandResponse::fail_msg("not implemented").await.into())
        }
        #[graphql(name = "changeIcon")]
        async fn change_icon(&self, ctx: &Context<'_>, #[graphql(name = "newIcon")] new_icon: String) -> async_graphql::Result<crate::operation::ResponseInterface> {
            let _ = (ctx, self, new_icon);
            Ok(crate::operation::CommandResponse::fail_msg("not implemented").await.into())
        }
        #[graphql(name = "addAttribute")]
        async fn add_attribute(&self, ctx: &Context<'_>, key: String, value: String, definition: String) -> async_graphql::Result<crate::operation::ResponseInterface> {
            let _ = (ctx, self, key, value, definition);
            Ok(crate::operation::CommandResponse::fail_msg("not implemented").await.into())
        }
        #[graphql(name = "removeAttribute")]
        async fn remove_attribute(&self, ctx: &Context<'_>, id: Id) -> async_graphql::Result<crate::operation::ResponseInterface> {
            let _ = (ctx, self, id);
            Ok(crate::operation::CommandResponse::fail_msg("not implemented").await.into())
        }
        #[graphql(name = "removeAttributes")]
        async fn remove_attributes(&self, ctx: &Context<'_>, ids: Vec<Id>) -> async_graphql::Result<crate::operation::ResponseInterface> {
            let _ = (ctx, self, ids);
            Ok(crate::operation::CommandResponse::fail_msg("not implemented").await.into())
        }
    }

    pub struct ConceptOperationInput""",
        """        #[graphql(name = "changeDescription")]
        async fn change_description(&self, ctx: &Context<'_>, #[graphql(name = "newDescription")] new_description: String) -> async_graphql::Result<crate::operation::ResponseInterface> {
            let rt = ctx.data::<Arc<ParentStore>>()?;
            Ok(dispatch_unsaved_kit_operation(
                rt,
                &self.change_id,
                crate::operation::Operation::ChangeDescription {
                    scope: Scope::Entity { entity_id: self.concept_id.clone() },
                    input: Input::Description { description: Some(new_description) },
                },
            )
            .await
            .into())
        }
        #[graphql(name = "changeIcon")]
        async fn change_icon(&self, ctx: &Context<'_>, #[graphql(name = "newIcon")] new_icon: String) -> async_graphql::Result<crate::operation::ResponseInterface> {
            let _ = (ctx, self, new_icon);
            Ok(crate::operation::CommandResponse::fail_msg("not implemented").await.into())
        }
        #[graphql(name = "addAttribute")]
        async fn add_attribute(&self, ctx: &Context<'_>, key: String, value: String, definition: String) -> async_graphql::Result<crate::operation::ResponseInterface> {
            let _ = (ctx, self, key, value, definition);
            Ok(crate::operation::CommandResponse::fail_msg("not implemented").await.into())
        }
        #[graphql(name = "removeAttribute")]
        async fn remove_attribute(&self, ctx: &Context<'_>, id: Id) -> async_graphql::Result<crate::operation::ResponseInterface> {
            let _ = (ctx, self, id);
            Ok(crate::operation::CommandResponse::fail_msg("not implemented").await.into())
        }
        #[graphql(name = "removeAttributes")]
        async fn remove_attributes(&self, ctx: &Context<'_>, ids: Vec<Id>) -> async_graphql::Result<crate::operation::ResponseInterface> {
            let _ = (ctx, self, ids);
            Ok(crate::operation::CommandResponse::fail_msg("not implemented").await.into())
        }
    }

    pub struct ConceptOperationInput""",
    ),
]

for old, new in pairs:
    if old not in t:
        raise SystemExit("block not found")
    t = t.replace(old, new, 1)

p.write_text(t, encoding="utf-8", newline="\n")
print("wired kit ops")
