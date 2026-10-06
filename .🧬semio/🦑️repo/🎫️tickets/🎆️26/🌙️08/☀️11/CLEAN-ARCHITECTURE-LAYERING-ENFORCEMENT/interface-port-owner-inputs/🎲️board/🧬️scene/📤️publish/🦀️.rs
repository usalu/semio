        /// 🎬️ Publishes prepared maps with one explicitly retained predecessor owner.
        pub fn synchronize_scene(&mut self,scene:super::BoardScenePublication)->Result<(),super::BoardSceneError>{
            let mut owner=scene.into_owner();
            if self.scene_retirement.is_some(){return Err(super::BoardSceneError::retain(NormalPortError::EventCredits,owner));}
            std::mem::swap(&mut self.nodes,&mut owner.prepared.nodes);
            std::mem::swap(&mut self.handles,&mut owner.prepared.handles);
            std::mem::swap(&mut self.edges,&mut owner.prepared.edges);
            std::mem::swap(&mut self.wires,&mut owner.prepared.wires);
            std::mem::swap(&mut self.regions,&mut owner.prepared.regions);
            std::mem::swap(&mut self.selection,&mut owner.prepared.selection);
            std::mem::swap(&mut self.preselect,&mut owner.prepared.preselect);
            std::mem::swap(&mut self.preselect_removed,&mut owner.prepared.preselect_removed);
            std::mem::swap(&mut self.selection_exit_highlight,&mut owner.prepared.selection_exit_highlight);
            self.port_mode=owner.mode;
            if !self.port_mode.has_ports(){self.selection_options.select_handles=false;}
            if let Some(camera)=owner.camera.as_ref(){self.set_camera(camera.x,camera.y,camera.zoom);}
            self.bump_content_scene_generation();
            self.scene_retirement=Some(Box::new(super::BoardSceneRetirement::new(owner)));
            Ok(())
        }

        /// ♻️ Transfers the predecessor and input descriptor to the caller's step scheduler.
        pub fn take_scene_retirement(&mut self)->Option<Box<super::BoardSceneRetirement>>{self.scene_retirement.take()}
