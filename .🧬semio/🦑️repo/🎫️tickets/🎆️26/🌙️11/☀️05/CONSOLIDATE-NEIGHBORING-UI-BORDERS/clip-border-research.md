# Clip Border Rendering

Production architecture energy clip frames use the shared browser layered UI. The capture readiness check previously required a complete original silhouette path, although shared border composition can split or omit segments. Readiness now checks that the border is no longer pending and its content clip matches measured geometry. No MP4, WebM, or MOV source assets exist under the architecture tree; clips are rendered on demand.
