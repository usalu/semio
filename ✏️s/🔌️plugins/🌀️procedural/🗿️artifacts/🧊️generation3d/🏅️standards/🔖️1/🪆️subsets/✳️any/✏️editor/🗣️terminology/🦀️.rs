//! 🗣️ Generation3d play app — the single `app_labels!` block plus the locale resolvers every taxonomy
//! node reaches for.

//#region 🔖️Labels
semio_framework_ui_locale::app_labels! {
    /// 🗣️ Complete UI label set for the 3D flow app; one field per label makes every locale combination compile-checked.
    pub struct Generation3dLabels {
        catalogue_inputs: native_en "Inputs", native_de "Eingaben", reuse_en "Inputs", reuse_de "Eingaben";
        catalogue_outputs: native_en "Outputs", native_de "Ausgaben", reuse_en "Outputs", reuse_de "Ausgaben";
        catalogue_input_slider: native_en "Slider", native_de "Schieberegler", reuse_en "Slider", reuse_de "Schieberegler";
        catalogue_input_note: native_en "Note", native_de "Notiz", reuse_en "Note", reuse_de "Notiz";
        catalogue_input_image: native_en "Image", native_de "Bild", reuse_en "Image", reuse_de "Bild";
        catalogue_variable: native_en "Variable", native_de "Variable", reuse_en "Variable", reuse_de "Variable";
        catalogue_output_preview: native_en "Preview", native_de "Vorschau", reuse_en "Preview", reuse_de "Vorschau";
        catalogue_output_action: native_en "Action", native_de "Aktion", reuse_en "Action", reuse_de "Aktion";
        catalogue_output_export: native_en "Export", native_de "Export", reuse_en "Export", reuse_de "Export";
        catalogue_brep_mesh_inspect_vertex: native_en "Inspect Mesh Vertex", native_de "Mesh-Punkt prüfen", reuse_en "Inspect Mesh Vertex", reuse_de "Mesh-Punkt prüfen";
        catalogue_brep_mesh_inspect_edge: native_en "Inspect Mesh Edge", native_de "Mesh-Kante prüfen", reuse_en "Inspect Mesh Edge", reuse_de "Mesh-Kante prüfen";
        catalogue_brep_mesh_inspect_face: native_en "Inspect Mesh Face", native_de "Mesh-Fläche prüfen", reuse_en "Inspect Mesh Face", reuse_de "Mesh-Fläche prüfen";
        catalogue_brep_mesh_bevel: native_en "Bevel Mesh Edges", native_de "Mesh-Kanten abschrägen", reuse_en "Bevel Mesh Edges", reuse_de "Mesh-Kanten abschrägen";
        catalogue_brep_mesh_dissolve_edges: native_en "Dissolve Mesh Edges", native_de "Mesh-Kanten auflösen", reuse_en "Dissolve Mesh Edges", reuse_de "Mesh-Kanten auflösen";
        catalogue_brep_mesh_dissolve_vertices: native_en "Dissolve Mesh Vertices", native_de "Mesh-Punkte auflösen", reuse_en "Dissolve Mesh Vertices", reuse_de "Mesh-Punkte auflösen";
        catalogue_brep_mesh_merge_vertices: native_en "Merge Mesh Vertices", native_de "Mesh-Punkte zusammenführen", reuse_en "Merge Mesh Vertices", reuse_de "Mesh-Punkte zusammenführen";
        catalogue_brep_mesh_move_proportional: native_en "Move Mesh Proportionally", native_de "Mesh proportional verschieben", reuse_en "Move Mesh Proportionally", reuse_de "Mesh proportional verschieben";
        catalogue_brep_mesh_snap_vertices: native_en "Snap Mesh Vertices to Grid", native_de "Mesh-Punkte am Raster ausrichten", reuse_en "Snap Mesh Vertices to Grid", reuse_de "Mesh-Punkte am Raster ausrichten";
        catalogue_brep_mesh_mirror: native_en "Mirror Mesh Half", native_de "Mesh-Hälfte spiegeln", reuse_en "Mirror Mesh Half", reuse_de "Mesh-Hälfte spiegeln";
        catalogue_brep_mesh_decimate: native_en "Simplify Mesh", native_de "Mesh vereinfachen", reuse_en "Simplify Mesh", reuse_de "Mesh vereinfachen";
        catalogue_brep_mesh_merge_coplanar: native_en "Merge Coplanar Mesh Faces", native_de "Koplanare Mesh-Flächen zusammenführen", reuse_en "Merge Coplanar Mesh Faces", reuse_de "Koplanare Mesh-Flächen zusammenführen";
        catalogue_brep_bool_compound_cut: native_en "Compound Cut", native_de "Zusammengesetzter Schnitt", reuse_en "Compound Cut", reuse_de "Zusammengesetzter Schnitt";
        catalogue_brep_bool_cut: native_en "Cut", native_de "Differenz", reuse_en "Cut", reuse_de "Differenz";
        catalogue_brep_bool_fuse: native_en "Fuse", native_de "Vereinigung", reuse_en "Fuse", reuse_de "Vereinigung";
        catalogue_brep_bool_intersect: native_en "Intersect", native_de "Schnittmenge", reuse_en "Intersect", reuse_de "Schnittmenge";
        catalogue_brep_brep: native_en "Brep", native_de "BRep", reuse_en "Brep", reuse_de "BRep";
        catalogue_brep_curve_approximate: native_en "Approximate", native_de "Annähern", reuse_en "Approximate", reuse_de "Annähern";
        catalogue_brep_curve_arc: native_en "Arc", native_de "Kreisbogen", reuse_en "Arc", reuse_de "Kreisbogen";
        catalogue_brep_curve_circle: native_en "Circle", native_de "Kreis", reuse_en "Circle", reuse_de "Kreis";
        catalogue_brep_curve_ellipse: native_en "Ellipse", native_de "Ellipse", reuse_en "Ellipse", reuse_de "Ellipse";
        catalogue_brep_curve_helix: native_en "Helix", native_de "Schraubenlinie", reuse_en "Helix", reuse_de "Schraubenlinie";
        catalogue_brep_curve_interpolate: native_en "Interpolate", native_de "Interpolieren", reuse_en "Interpolate", reuse_de "Interpolieren";
        catalogue_brep_curve_line: native_en "Line", native_de "Linie", reuse_en "Line", reuse_de "Linie";
        catalogue_brep_curve_polygon: native_en "Polygon", native_de "Vieleck", reuse_en "Polygon", reuse_de "Vieleck";
        catalogue_brep_curve_polyline: native_en "Polyline", native_de "Linienzug", reuse_en "Polyline", reuse_de "Linienzug";
        catalogue_brep_curve_rectangle: native_en "Rectangle", native_de "Rechteck", reuse_en "Rectangle", reuse_de "Rechteck";
        catalogue_brep_edge: native_en "Edge", native_de "Kante", reuse_en "Edge", reuse_de "Kante";
        catalogue_brep_eval_curve_closest_parameter: native_en "Curve Closest Parameter", native_de "Nächster Kurvenparameter", reuse_en "Curve Closest Parameter", reuse_de "Nächster Kurvenparameter";
        catalogue_brep_eval_curve_curvature: native_en "Curve Curvature", native_de "Kurvenkrümmung", reuse_en "Curve Curvature", reuse_de "Kurvenkrümmung";
        catalogue_brep_eval_curve_domain: native_en "Curve Domain", native_de "Kurvenbereich", reuse_en "Curve Domain", reuse_de "Kurvenbereich";
        catalogue_brep_eval_curve_point: native_en "Curve Point", native_de "Kurvenpunkt", reuse_en "Curve Point", reuse_de "Kurvenpunkt";
        catalogue_brep_eval_curve_tangent: native_en "Curve Tangent", native_de "Kurventangente", reuse_en "Curve Tangent", reuse_de "Kurventangente";
        catalogue_brep_eval_surf_normal: native_en "Surface Normal", native_de "Flächennormale", reuse_en "Surface Normal", reuse_de "Flächennormale";
        catalogue_brep_eval_surf_point: native_en "Surface Point", native_de "Flächenpunkt", reuse_en "Surface Point", reuse_de "Flächenpunkt";
        catalogue_brep_eval_surface_closest_uv: native_en "Surface Closest Uv", native_de "Nächste UV-Koordinaten", reuse_en "Surface Closest Uv", reuse_de "Nächste UV-Koordinaten";
        catalogue_brep_face: native_en "Face", native_de "Fläche", reuse_en "Face", reuse_de "Fläche";
        catalogue_brep_geometry: native_en "Geometry", native_de "Geometrie", reuse_en "Geometry", reuse_de "Geometrie";
        catalogue_brep_intersect_curve_curve: native_en "Curve Curve", native_de "Kurve mit Kurve schneiden", reuse_en "Curve Curve", reuse_de "Kurve mit Kurve schneiden";
        catalogue_brep_intersect_curve_surface: native_en "Curve Surface", native_de "Kurve mit Fläche schneiden", reuse_en "Curve Surface", reuse_de "Kurve mit Fläche schneiden";
        catalogue_brep_intersect_section: native_en "Section", native_de "Querschnitt", reuse_en "Section", reuse_de "Querschnitt";
        catalogue_brep_intersect_split: native_en "Split", native_de "Teilen", reuse_en "Split", reuse_de "Teilen";
        catalogue_brep_intersect_surface_surface: native_en "Surface Surface", native_de "Flächen miteinander schneiden", reuse_en "Surface Surface", reuse_de "Flächen miteinander schneiden";
        catalogue_brep_io_export_dwg: native_en "Export Dwg", native_de "DWG exportieren", reuse_en "Export Dwg", reuse_de "DWG exportieren";
        catalogue_brep_io_export_obj: native_en "Export Obj", native_de "OBJ exportieren", reuse_en "Export Obj", reuse_de "OBJ exportieren";
        catalogue_brep_io_export_step: native_en "Export Step", native_de "STEP exportieren", reuse_en "Export Step", reuse_de "STEP exportieren";
        catalogue_brep_io_export_stl: native_en "Export Stl", native_de "STL exportieren", reuse_en "Export Stl", reuse_de "STL exportieren";
        catalogue_brep_io_import_dwg: native_en "Import Dwg", native_de "DWG importieren", reuse_en "Import Dwg", reuse_de "DWG importieren";
        catalogue_brep_io_import_obj: native_en "Import Obj", native_de "OBJ importieren", reuse_en "Import Obj", reuse_de "OBJ importieren";
        catalogue_brep_io_import_step: native_en "Import Step", native_de "STEP importieren", reuse_en "Import Step", reuse_de "STEP importieren";
        catalogue_brep_io_import_stl: native_en "Import Stl", native_de "STL importieren", reuse_en "Import Stl", reuse_de "STL importieren";
        catalogue_brep_measure_area: native_en "Area", native_de "Flächeninhalt", reuse_en "Area", reuse_de "Flächeninhalt";
        catalogue_brep_measure_bounding_box: native_en "Bounding Box", native_de "Begrenzungsquader", reuse_en "Bounding Box", reuse_de "Begrenzungsquader";
        catalogue_brep_measure_center_of_mass: native_en "Center Of Mass", native_de "Schwerpunkt", reuse_en "Center Of Mass", reuse_de "Schwerpunkt";
        catalogue_brep_measure_classify: native_en "Classify", native_de "Klassifizieren", reuse_en "Classify", reuse_de "Klassifizieren";
        catalogue_brep_measure_closest_point: native_en "Closest Point", native_de "Nächster Punkt", reuse_en "Closest Point", reuse_de "Nächster Punkt";
        catalogue_brep_measure_distance: native_en "Distance", native_de "Abstand", reuse_en "Distance", reuse_de "Abstand";
        catalogue_brep_measure_length: native_en "Length", native_de "Länge", reuse_en "Length", reuse_de "Länge";
        catalogue_brep_measure_validate: native_en "Validate", native_de "Validieren", reuse_en "Validate", reuse_de "Validieren";
        catalogue_brep_measure_volume: native_en "Volume", native_de "Volumen", reuse_en "Volume", reuse_de "Volumen";
        catalogue_brep_mesh: native_en "Polygon Mesh", native_de "Polygonnetz", reuse_en "Polygon Mesh", reuse_de "Polygonnetz";
        catalogue_brep_mesh_analyze: native_en "Analyze Mesh", native_de "Mesh analysieren", reuse_en "Analyze Mesh", reuse_de "Mesh analysieren";
        catalogue_brep_mesh_box: native_en "Mesh Box", native_de "Mesh-Quader", reuse_en "Mesh Box", reuse_de "Mesh-Quader";
        catalogue_brep_mesh_cone: native_en "Mesh Cone", native_de "Mesh-Kegel", reuse_en "Mesh Cone", reuse_de "Mesh-Kegel";
        catalogue_brep_mesh_construct: native_en "Construct Mesh", native_de "Mesh konstruieren", reuse_en "Construct Mesh", reuse_de "Mesh konstruieren";
        catalogue_brep_mesh_cylinder: native_en "Mesh Cylinder", native_de "Mesh-Zylinder", reuse_en "Mesh Cylinder", reuse_de "Mesh-Zylinder";
        catalogue_brep_mesh_delete_faces: native_en "Delete Mesh Faces", native_de "Mesh-Flächen löschen", reuse_en "Delete Mesh Faces", reuse_de "Mesh-Flächen löschen";
        catalogue_brep_mesh_export_json: native_en "Mesh to JSON", native_de "Mesh als JSON", reuse_en "Mesh to JSON", reuse_de "Mesh als JSON";
        catalogue_brep_mesh_export_obj: native_en "Mesh to OBJ", native_de "Mesh als OBJ", reuse_en "Mesh to OBJ", reuse_de "Mesh als OBJ";
        catalogue_brep_mesh_extrude: native_en "Extrude Mesh Faces", native_de "Mesh-Flächen extrudieren", reuse_en "Extrude Mesh Faces", reuse_de "Mesh-Flächen extrudieren";
        catalogue_brep_mesh_fill_holes: native_en "Fill Mesh Holes", native_de "Mesh-Löcher schließen", reuse_en "Fill Mesh Holes", reuse_de "Mesh-Löcher schließen";
        catalogue_brep_mesh_flip: native_en "Flip Mesh Faces", native_de "Mesh-Flächen umdrehen", reuse_en "Flip Mesh Faces", reuse_de "Mesh-Flächen umdrehen";
        catalogue_brep_mesh_from_brep: native_en "Mesh from B-Rep", native_de "Mesh aus BRep", reuse_en "Mesh from B-Rep", reuse_de "Mesh aus BRep";
        catalogue_brep_mesh_inset: native_en "Inset Mesh Faces", native_de "Mesh-Flächen einrücken", reuse_en "Inset Mesh Faces", reuse_de "Mesh-Flächen einrücken";
        catalogue_brep_mesh_knife_cut: native_en "Knife Cut Mesh Face", native_de "Mesh-Fläche schneiden", reuse_en "Knife Cut Mesh Face", reuse_de "Mesh-Fläche schneiden";
        catalogue_brep_mesh_loop_cut: native_en "Cut Mesh Loops", native_de "Mesh-Kantenschleifen schneiden", reuse_en "Cut Mesh Loops", reuse_de "Mesh-Kantenschleifen schneiden";
        catalogue_brep_mesh_move_vertices: native_en "Move Mesh Vertices", native_de "Mesh-Punkte verschieben", reuse_en "Move Mesh Vertices", reuse_de "Mesh-Punkte verschieben";
        catalogue_brep_mesh_orient: native_en "Orient Mesh Faces", native_de "Mesh-Flächen ausrichten", reuse_en "Orient Mesh Faces", reuse_de "Mesh-Flächen ausrichten";
        catalogue_brep_mesh_plane: native_en "Mesh Plane", native_de "Mesh-Ebene", reuse_en "Mesh Plane", reuse_de "Mesh-Ebene";
        catalogue_brep_mesh_rotate: native_en "Rotate Mesh", native_de "Mesh drehen", reuse_en "Rotate Mesh", reuse_de "Mesh drehen";
        catalogue_brep_mesh_rotate_components: native_en "Rotate Mesh Components", native_de "Mesh-Komponenten drehen", reuse_en "Rotate Mesh Components", reuse_de "Mesh-Komponenten drehen";
        catalogue_brep_mesh_scale: native_en "Scale Mesh", native_de "Mesh skalieren", reuse_en "Scale Mesh", reuse_de "Mesh skalieren";
        catalogue_brep_mesh_scale_components: native_en "Scale Mesh Components", native_de "Mesh-Komponenten skalieren", reuse_en "Scale Mesh Components", reuse_de "Mesh-Komponenten skalieren";
        catalogue_brep_mesh_sphere: native_en "Mesh Sphere", native_de "Mesh-Kugel", reuse_en "Mesh Sphere", reuse_de "Mesh-Kugel";
        catalogue_brep_mesh_subdivide: native_en "Subdivide Mesh Faces", native_de "Mesh-Flächen unterteilen", reuse_en "Subdivide Mesh Faces", reuse_de "Mesh-Flächen unterteilen";
        catalogue_brep_mesh_to_brep: native_en "Faceted B-Rep from Mesh", native_de "Facettierter BRep aus Mesh", reuse_en "Faceted B-Rep from Mesh", reuse_de "Facettierter BRep aus Mesh";
        catalogue_brep_mesh_translate: native_en "Translate Mesh", native_de "Mesh verschieben", reuse_en "Translate Mesh", reuse_de "Mesh verschieben";
        catalogue_brep_mesh_translate_components: native_en "Move Mesh Components", native_de "Mesh-Komponenten verschieben", reuse_en "Move Mesh Components", reuse_de "Mesh-Komponenten verschieben";
        catalogue_brep_mesh_triangulate: native_en "Triangulate Mesh", native_de "Mesh triangulieren", reuse_en "Triangulate Mesh", reuse_de "Mesh triangulieren";
        catalogue_brep_mesh_weld: native_en "Weld Mesh Vertices", native_de "Mesh-Punkte verschweißen", reuse_en "Weld Mesh Vertices", reuse_de "Mesh-Punkte verschweißen";
        catalogue_brep_prim3d_box: native_en "Box", native_de "Quader", reuse_en "Box", reuse_de "Quader";
        catalogue_brep_prim3d_cone: native_en "Cone", native_de "Kegel", reuse_en "Cone", reuse_de "Kegel";
        catalogue_brep_prim3d_convex_hull: native_en "Convex Hull", native_de "Konvexe Hülle", reuse_en "Convex Hull", reuse_de "Konvexe Hülle";
        catalogue_brep_prim3d_cylinder: native_en "Cylinder", native_de "Zylinder", reuse_en "Cylinder", reuse_de "Zylinder";
        catalogue_brep_prim3d_sphere: native_en "Sphere", native_de "Kugel", reuse_en "Sphere", reuse_de "Kugel";
        catalogue_brep_prim3d_torus: native_en "Torus", native_de "Torus", reuse_en "Torus", reuse_de "Torus";
        catalogue_brep_solid_chamfer: native_en "Chamfer", native_de "Abschrägen", reuse_en "Chamfer", reuse_de "Abschrägen";
        catalogue_brep_solid_chamfer_asymmetric: native_en "Asymmetric Chamfer", native_de "Asymmetrisch abschrägen", reuse_en "Asymmetric Chamfer", reuse_de "Asymmetrisch abschrägen";
        catalogue_brep_solid_chamfer_edges: native_en "Chamfer Edges", native_de "Kanten abschrägen", reuse_en "Chamfer Edges", reuse_de "Kanten abschrägen";
        catalogue_brep_solid_defeature: native_en "Defeature", native_de "Formelemente entfernen", reuse_en "Defeature", reuse_de "Formelemente entfernen";
        catalogue_brep_solid_draft: native_en "Draft", native_de "Formschräge", reuse_en "Draft", reuse_de "Formschräge";
        catalogue_brep_solid_extrude: native_en "Extrude Curve", native_de "Kurve extrudieren", reuse_en "Extrude Curve", reuse_de "Kurve extrudieren";
        catalogue_brep_solid_fillet: native_en "Fillet", native_de "Abrunden", reuse_en "Fillet", reuse_de "Abrunden";
        catalogue_brep_solid_fillet_edges: native_en "Fillet Edges", native_de "Kanten abrunden", reuse_en "Fillet Edges", reuse_de "Kanten abrunden";
        catalogue_brep_solid_fillet_variable: native_en "Variable Fillet", native_de "Variabel abrunden", reuse_en "Variable Fillet", reuse_de "Variabel abrunden";
        catalogue_brep_solid_offset_solid: native_en "Offset Solid", native_de "Körper versetzen", reuse_en "Offset Solid", reuse_de "Körper versetzen";
        catalogue_brep_solid_shell: native_en "Shell", native_de "Aushöhlen", reuse_en "Shell", reuse_de "Aushöhlen";
        catalogue_brep_surf_coons: native_en "Coons Patch", native_de "Coons-Fläche", reuse_en "Coons Patch", reuse_de "Coons-Fläche";
        catalogue_brep_surf_nurbs_grid: native_en "Nurbs Grid", native_de "NURBS-Gitter", reuse_en "Nurbs Grid", reuse_de "NURBS-Gitter";
        catalogue_brep_surf_offset: native_en "Offset Face", native_de "Fläche versetzen", reuse_en "Offset Face", reuse_de "Fläche versetzen";
        catalogue_brep_surf_planar_face: native_en "Planar Face", native_de "Ebene Fläche", reuse_en "Planar Face", reuse_de "Ebene Fläche";
        catalogue_brep_surf_planar_face_wire: native_en "Planar Face Wire", native_de "Ebene Fläche aus Linienzug", reuse_en "Planar Face Wire", reuse_de "Ebene Fläche aus Linienzug";
        catalogue_brep_surf_plane: native_en "Plane", native_de "Ebene", reuse_en "Plane", reuse_de "Ebene";
        catalogue_brep_surf_thicken: native_en "Thicken", native_de "Verdicken", reuse_en "Thicken", reuse_de "Verdicken";
        catalogue_brep_sweep_extrude: native_en "Extrude", native_de "Extrudieren", reuse_en "Extrude", reuse_de "Extrudieren";
        catalogue_brep_sweep_helical: native_en "Helical Sweep", native_de "Schraubenprofil ziehen", reuse_en "Helical Sweep", reuse_de "Schraubenprofil ziehen";
        catalogue_brep_sweep_loft: native_en "Loft", native_de "Profile verbinden", reuse_en "Loft", reuse_de "Profile verbinden";
        catalogue_brep_sweep_pipe: native_en "Pipe", native_de "Rohr entlang Pfad", reuse_en "Pipe", reuse_de "Rohr entlang Pfad";
        catalogue_brep_sweep_revolve: native_en "Revolve", native_de "Rotationskörper", reuse_en "Revolve", reuse_de "Rotationskörper";
        catalogue_brep_sweep_sweep: native_en "Sweep", native_de "Profil entlang Pfad ziehen", reuse_en "Sweep", reuse_de "Profil entlang Pfad ziehen";
        catalogue_brep_text: native_en "Text", native_de "Text", reuse_en "Text", reuse_de "Text";
        catalogue_brep_topology_compound: native_en "Compound", native_de "Zusammensetzen", reuse_en "Compound", reuse_de "Zusammensetzen";
        catalogue_brep_topology_explode: native_en "Explode", native_de "Zerlegen", reuse_en "Explode", reuse_de "Zerlegen";
        catalogue_brep_topology_label: native_en "Label", native_de "Beschriften", reuse_en "Label", reuse_de "Beschriften";
        catalogue_brep_topology_shells: native_en "Shells", native_de "Schalen", reuse_en "Shells", reuse_de "Schalen";
        catalogue_brep_util_convert_to_nurbs: native_en "Convert To Nurbs", native_de "In NURBS umwandeln", reuse_en "Convert To Nurbs", reuse_de "In NURBS umwandeln";
        catalogue_brep_util_face_from_wire: native_en "Face From Wire", native_de "Fläche aus Linienzug", reuse_en "Face From Wire", reuse_de "Fläche aus Linienzug";
        catalogue_brep_util_heal: native_en "Heal", native_de "Reparieren", reuse_en "Heal", reuse_de "Reparieren";
        catalogue_brep_util_sew: native_en "Sew", native_de "Vernähen", reuse_en "Sew", reuse_de "Vernähen";
        catalogue_brep_util_vertex: native_en "Vertex", native_de "Punkt", reuse_en "Vertex", reuse_de "Punkt";
        catalogue_brep_vertex: native_en "Vertex", native_de "Punkt", reuse_en "Vertex", reuse_de "Punkt";
        catalogue_brep_xform_circular_pattern: native_en "Circular Pattern", native_de "Kreisförmiges Muster", reuse_en "Circular Pattern", reuse_de "Kreisförmiges Muster";
        catalogue_brep_xform_copy: native_en "Copy", native_de "Kopieren", reuse_en "Copy", reuse_de "Kopieren";
        catalogue_brep_xform_grid_pattern: native_en "Grid Pattern", native_de "Rastermuster", reuse_en "Grid Pattern", reuse_de "Rastermuster";
        catalogue_brep_xform_linear_pattern: native_en "Linear Pattern", native_de "Lineares Muster", reuse_en "Linear Pattern", reuse_de "Lineares Muster";
        catalogue_brep_xform_mirror: native_en "Mirror", native_de "Spiegeln", reuse_en "Mirror", reuse_de "Spiegeln";
        catalogue_brep_xform_rotate: native_en "Rotate", native_de "Drehen", reuse_en "Rotate", reuse_de "Drehen";
        catalogue_brep_xform_rotate_about: native_en "Rotate About", native_de "Um Punkt drehen", reuse_en "Rotate About", reuse_de "Um Punkt drehen";
        catalogue_brep_xform_scale: native_en "Scale", native_de "Skalieren", reuse_en "Scale", reuse_de "Skalieren";
        catalogue_brep_xform_translate: native_en "Translate", native_de "Verschieben", reuse_en "Translate", reuse_de "Verschieben";
        catalogue_input: native_en "Input", native_de "Eingang", reuse_en "Input", reuse_de "Eingang";
        catalogue_output: native_en "Output", native_de "Ausgang", reuse_en "Output", reuse_de "Ausgang";
        catalogue_contract: native_en "Contract", native_de "Schnittstelle", reuse_en "Contract", reuse_de "Schnittstelle";
        catalogue_brep: native_en "Brep", native_de "BRep", reuse_en "Brep", reuse_de "BRep";
        input_name_a: native_en "A", native_de "A", reuse_en "A", reuse_de "A";
        input_name_b: native_en "B", native_de "B", reuse_en "B", reuse_de "B";
        input_name_amount: native_en "Amount", native_de "Betrag", reuse_en "Amount", reuse_de "Betrag";
        input_name_angle: native_en "Angle", native_de "Winkel", reuse_en "Angle", reuse_de "Winkel";
        input_name_axis: native_en "Axis", native_de "Achse", reuse_en "Axis", reuse_de "Achse";
        input_name_axis_direction: native_en "Axis Direction", native_de "Achsenrichtung", reuse_en "Axis Direction", reuse_de "Achsenrichtung";
        input_name_axis_origin: native_en "Axis Origin", native_de "Achsenursprung", reuse_en "Axis Origin", reuse_de "Achsenursprung";
        input_name_brep: native_en "BRep", native_de "BRep", reuse_en "BRep", reuse_de "BRep";
        input_name_center: native_en "Center", native_de "Mittelpunkt", reuse_en "Center", reuse_de "Mittelpunkt";
        input_name_compound: native_en "Compound", native_de "Verbund", reuse_en "Compound", reuse_de "Verbund";
        input_name_control_points: native_en "Control Points", native_de "Kontrollpunkte", reuse_en "Control Points", reuse_de "Kontrollpunkte";
        input_name_count: native_en "Count", native_de "Anzahl", reuse_en "Count", reuse_de "Anzahl";
        input_name_count_x: native_en "X Count", native_de "Anzahl in X", reuse_en "X Count", reuse_de "Anzahl in X";
        input_name_count_y: native_en "Y Count", native_de "Anzahl in Y", reuse_en "Y Count", reuse_de "Anzahl in Y";
        input_name_curve: native_en "Curve", native_de "Kurve", reuse_en "Curve", reuse_de "Kurve";
        input_name_curves: native_en "Curves", native_de "Kurven", reuse_en "Curves", reuse_de "Kurven";
        input_name_cuts: native_en "Cuts", native_de "Schnitte", reuse_en "Cuts", reuse_de "Schnitte";
        input_name_d1: native_en "First Distance", native_de "Erster Abstand", reuse_en "First Distance", reuse_de "Erster Abstand";
        input_name_d2: native_en "Second Distance", native_de "Zweiter Abstand", reuse_en "Second Distance", reuse_de "Zweiter Abstand";
        input_name_data: native_en "Data", native_de "Daten", reuse_en "Data", reuse_de "Daten";
        input_name_deflection: native_en "Deflection", native_de "Abweichung", reuse_en "Deflection", reuse_de "Abweichung";
        input_name_degree: native_en "Degree", native_de "Grad", reuse_en "Degree", reuse_de "Grad";
        input_name_degree_u: native_en "U Degree", native_de "Grad in U", reuse_en "U Degree", reuse_de "Grad in U";
        input_name_degree_v: native_en "V Degree", native_de "Grad in V", reuse_en "V Degree", reuse_de "Grad in V";
        input_name_depth: native_en "Depth", native_de "Tiefe", reuse_en "Depth", reuse_de "Tiefe";
        input_name_dir_x: native_en "X Direction", native_de "X-Richtung", reuse_en "X Direction", reuse_de "X-Richtung";
        input_name_dir_y: native_en "Y Direction", native_de "Y-Richtung", reuse_en "Y Direction", reuse_de "Y-Richtung";
        input_name_direction: native_en "Direction", native_de "Richtung", reuse_en "Direction", reuse_de "Richtung";
        input_name_distance: native_en "Distance", native_de "Abstand", reuse_en "Distance", reuse_de "Abstand";
        input_name_edge: native_en "Edge", native_de "Kante", reuse_en "Edge", reuse_de "Kante";
        input_name_edges: native_en "Edges", native_de "Kanten", reuse_en "Edges", reuse_de "Kanten";
        input_name_end: native_en "End", native_de "Ende", reuse_en "End", reuse_de "Ende";
        input_name_end_angle: native_en "End Angle", native_de "Endwinkel", reuse_en "End Angle", reuse_de "Endwinkel";
        input_name_face: native_en "Face", native_de "Fläche", reuse_en "Face", reuse_de "Fläche";
        input_name_faces: native_en "Faces", native_de "Flächen", reuse_en "Faces", reuse_de "Flächen";
        input_name_factor: native_en "Factor", native_de "Faktor", reuse_en "Factor", reuse_de "Faktor";
        input_name_geometry: native_en "Geometry", native_de "Geometrie", reuse_en "Geometry", reuse_de "Geometrie";
        input_name_guide: native_en "Guide", native_de "Führung", reuse_en "Guide", reuse_de "Führung";
        input_name_handle: native_en "Handle", native_de "Referenz", reuse_en "Handle", reuse_de "Referenz";
        input_name_height: native_en "Height", native_de "Höhe", reuse_en "Height", reuse_de "Höhe";
        input_name_index: native_en "Index", native_de "Index", reuse_en "Index", reuse_de "Index";
        input_name_kind: native_en "Kind", native_de "Art", reuse_en "Kind", reuse_de "Art";
        input_name_major: native_en "Major Radius", native_de "Großer Radius", reuse_en "Major Radius", reuse_de "Großer Radius";
        input_name_mesh: native_en "Mesh", native_de "Mesh", reuse_en "Mesh", reuse_de "Mesh";
        input_name_minor: native_en "Minor Radius", native_de "Kleiner Radius", reuse_en "Minor Radius", reuse_de "Kleiner Radius";
        input_name_mode: native_en "Mode", native_de "Modus", reuse_en "Mode", reuse_de "Modus";
        input_name_neutral_point: native_en "Neutral Point", native_de "Neutralpunkt", reuse_en "Neutral Point", reuse_de "Neutralpunkt";
        input_name_normal: native_en "Normal", native_de "Normale", reuse_en "Normal", reuse_de "Normale";
        input_name_offset: native_en "Offset", native_de "Versatz", reuse_en "Offset", reuse_de "Versatz";
        input_name_open_faces: native_en "Open Faces", native_de "Offene Flächen", reuse_en "Open Faces", reuse_de "Offene Flächen";
        input_name_origin: native_en "Origin", native_de "Ursprung", reuse_en "Origin", reuse_de "Ursprung";
        input_name_parameter: native_en "Parameter", native_de "Parameter", reuse_en "Parameter", reuse_de "Parameter";
        input_name_path: native_en "Path", native_de "Pfad", reuse_en "Path", reuse_de "Pfad";
        input_name_pitch: native_en "Pitch", native_de "Steigung", reuse_en "Pitch", reuse_de "Steigung";
        input_name_pivot: native_en "Pivot", native_de "Drehpunkt", reuse_en "Pivot", reuse_de "Drehpunkt";
        input_name_plane_normal: native_en "Plane Normal", native_de "Ebenennormale", reuse_en "Plane Normal", reuse_de "Ebenennormale";
        input_name_plane_origin: native_en "Plane Origin", native_de "Ebenenursprung", reuse_en "Plane Origin", reuse_de "Ebenenursprung";
        input_name_point: native_en "Point", native_de "Punkt", reuse_en "Point", reuse_de "Punkt";
        input_name_points: native_en "Points", native_de "Punkte", reuse_en "Points", reuse_de "Punkte";
        input_name_preview: native_en "Preview", native_de "Vorschau", reuse_en "Preview", reuse_de "Vorschau";
        input_name_profile: native_en "Profile", native_de "Profil", reuse_en "Profile", reuse_de "Profil";
        input_name_profiles: native_en "Profiles", native_de "Profile", reuse_en "Profiles", reuse_de "Profile";
        input_name_pull_direction: native_en "Pull Direction", native_de "Zugrichtung", reuse_en "Pull Direction", reuse_de "Zugrichtung";
        input_name_radius: native_en "Radius", native_de "Radius", reuse_en "Radius", reuse_de "Radius";
        input_name_radius_end: native_en "End Radius", native_de "Endradius", reuse_en "End Radius", reuse_de "Endradius";
        input_name_radius_start: native_en "Start Radius", native_de "Anfangsradius", reuse_en "Start Radius", reuse_de "Anfangsradius";
        input_name_rows: native_en "Rows", native_de "Zeilen", reuse_en "Rows", reuse_de "Zeilen";
        input_name_segments: native_en "Segments", native_de "Segmente", reuse_en "Segments", reuse_de "Segmente";
        input_name_selection: native_en "Selection", native_de "Auswahl", reuse_en "Selection", reuse_de "Auswahl";
        input_name_semi_major: native_en "Major Semiaxis", native_de "Große Halbachse", reuse_en "Major Semiaxis", reuse_de "Große Halbachse";
        input_name_semi_minor: native_en "Minor Semiaxis", native_de "Kleine Halbachse", reuse_en "Minor Semiaxis", reuse_de "Kleine Halbachse";
        input_name_sides: native_en "Sides", native_de "Seiten", reuse_en "Sides", reuse_de "Seiten";
        input_name_smooth: native_en "Smooth", native_de "Glätten", reuse_en "Smooth", reuse_de "Glätten";
        input_name_solid: native_en "Solid", native_de "Körper", reuse_en "Solid", reuse_de "Körper";
        input_name_solids: native_en "Solids", native_de "Körper", reuse_en "Solids", reuse_de "Körper";
        input_name_spacing: native_en "Spacing", native_de "Teilung", reuse_en "Spacing", reuse_de "Teilung";
        input_name_spacing_x: native_en "X Spacing", native_de "Teilung in X", reuse_en "X Spacing", reuse_de "Teilung in X";
        input_name_spacing_y: native_en "Y Spacing", native_de "Teilung in Y", reuse_en "Y Spacing", reuse_de "Teilung in Y";
        input_name_start: native_en "Start", native_de "Anfang", reuse_en "Start", reuse_de "Anfang";
        input_name_start_angle: native_en "Start Angle", native_de "Anfangswinkel", reuse_en "Start Angle", reuse_de "Anfangswinkel";
        input_name_subdivisions: native_en "Subdivisions", native_de "Unterteilungen", reuse_en "Subdivisions", reuse_de "Unterteilungen";
        input_name_surface: native_en "Surface", native_de "Oberfläche", reuse_en "Surface", reuse_de "Oberfläche";
        input_name_target: native_en "Target", native_de "Ziel", reuse_en "Target", reuse_de "Ziel";
        input_name_text: native_en "Text", native_de "Text", reuse_en "Text", reuse_de "Text";
        input_name_thickness: native_en "Thickness", native_de "Dicke", reuse_en "Thickness", reuse_de "Dicke";
        input_name_tolerance: native_en "Tolerance", native_de "Toleranz", reuse_en "Tolerance", reuse_de "Toleranz";
        input_name_tools: native_en "Tools", native_de "Werkzeuge", reuse_en "Tools", reuse_de "Werkzeuge";
        input_name_turns: native_en "Turns", native_de "Windungen", reuse_en "Turns", reuse_de "Windungen";
        input_name_u: native_en "U", native_de "U", reuse_en "U", reuse_de "U";
        input_name_v: native_en "V", native_de "V", reuse_en "V", reuse_de "V";
        input_name_value: native_en "Value", native_de "Wert", reuse_en "Value", reuse_de "Wert";
        input_name_vector: native_en "Vector", native_de "Vektor", reuse_en "Vector", reuse_de "Vektor";
        input_name_vertex: native_en "Vertex", native_de "Punkt", reuse_en "Vertex", reuse_de "Punkt";
        input_name_vertices: native_en "Vertices", native_de "Punkte", reuse_en "Vertices", reuse_de "Punkte";
        input_name_width: native_en "Width", native_de "Breite", reuse_en "Width", reuse_de "Breite";
        input_name_wire: native_en "Wire", native_de "Linienzug", reuse_en "Wire", reuse_de "Linienzug";
        input_name_grid: native_en "Grid", native_de "Raster", reuse_en "Grid", reuse_de "Raster";
        input_name_ratio: native_en "Ratio", native_de "Verhältnis", reuse_en "Ratio", reuse_de "Verhältnis";
        input_name_components: native_en "Components", native_de "Komponenten", reuse_en "Components", reuse_de "Komponenten";
        input_name_granularity: native_en "Element Type", native_de "Elementart", reuse_en "Element Type", reuse_de "Elementart";
        input_name_x: native_en "X", native_de "X", reuse_en "X", reuse_de "X";
        input_name_y: native_en "Y", native_de "Y", reuse_en "Y", reuse_de "Y";
        input_name_z: native_en "Z", native_de "Z", reuse_en "Z", reuse_de "Z";
        input_name_number: native_en "Number", native_de "Zahl", reuse_en "Number", reuse_de "Zahl";
        input_name_numbers: native_en "Numbers", native_de "Zahlen", reuse_en "Numbers", reuse_de "Zahlen";
        input_name_axis_name: native_en "Axis", native_de "Achse", reuse_en "Axis", reuse_de "Achse";
        widgets: native_en "Widgets", native_de "Elemente", reuse_en "Widgets", reuse_de "Elemente";
        schema_prefix: native_en "Schema:", native_de "Schema:", reuse_en "Schema:", reuse_de "Schema:";
        widgets_prefix: native_en "Widgets:", native_de "Elemente:", reuse_en "Widgets:", reuse_de "Elemente:";
        no_selection: native_en "No selection", native_de "Keine Auswahl", reuse_en "No selection", reuse_de "Keine Auswahl";
        id_field: native_en "Id", native_de "ID", reuse_en "Id", reuse_de "ID";
        value_field: native_en "Value", native_de "Wert", reuse_en "Value", reuse_de "Wert";
        range_field: native_en "Range", native_de "Bereich", reuse_en "Range", reuse_de "Bereich";
        widget_group: native_en "Widget", native_de "Element", reuse_en "Widget", reuse_de "Element";
        input_connected: native_en "Connected", native_de "Verbunden", reuse_en "Connected", reuse_de "Verbunden";
        input_connect_hint: native_en "Connect an output", native_de "Ausgang verbinden", reuse_en "Connect an output", reuse_de "Ausgang verbinden";
        input_text_too_long: native_en "Edit large text in the graph source", native_de "Langen Text im Quellknoten bearbeiten", reuse_en "Edit large text in the graph source", reuse_de "Langen Text im Quellknoten bearbeiten";
        generate_hint: native_en "Add a generation to edit input values.", native_de "Erstelle eine Generation, um Eingabewerte zu bearbeiten.", reuse_en "Add a generation to edit input values.", reuse_de "Erstelle eine Generation, um Eingabewerte zu bearbeiten.";
        preview_hint: native_en "(evaluate a generation to preview output)", native_de "(Generation auswerten, um die Ausgabe in der Vorschau zu sehen)", reuse_en "(evaluate a generation to preview output)", reuse_de "(Generation auswerten, um die Ausgabe in der Vorschau zu sehen)";
        window_flow: native_en "Flow", native_de "Workflow", reuse_en "Flow", reuse_de "Workflow";
        window_preview: native_en "Preview", native_de "Vorschau", reuse_en "Preview", reuse_de "Vorschau";
        window_generations: native_en "Generations", native_de "Generationen", reuse_en "Generations", reuse_de "Generationen";
        window_generate_form: native_en "Form", native_de "Formular", reuse_en "Form", reuse_de "Formular";
        window_generate_preview: native_en "Preview", native_de "Vorschau", reuse_en "Preview", reuse_de "Vorschau";
        delete_selection: native_en "Delete selection", native_de "Auswahl löschen", reuse_en "Delete selection", reuse_de "Auswahl löschen";
        graph_nodes: native_en "Nodes", native_de "Knoten", reuse_en "Nodes", reuse_de "Knoten";
        graph_wires: native_en "Wires", native_de "Leitungen", reuse_en "Wires", reuse_de "Leitungen";
        graph_input_port: native_en "Input", native_de "Eingang", reuse_en "Input", reuse_de "Eingang";
        graph_output_port: native_en "Output", native_de "Ausgang", reuse_en "Output", reuse_de "Ausgang";
        graph_empty: native_en "(no nodes)", native_de "(keine Knoten)", reuse_en "(no nodes)", reuse_de "(keine Knoten)";
        graph_unwired: native_en "(no wires)", native_de "(keine Leitungen)", reuse_en "(no wires)", reuse_de "(keine Leitungen)";
        status_ok: native_en "Evaluated", native_de "Ausgewertet", reuse_en "Evaluated", reuse_de "Ausgewertet";
        status_queued: native_en "Queued", native_de "In Warteschlange", reuse_en "Queued", reuse_de "In Warteschlange";
        status_computing: native_en "Computing", native_de "Berechnen", reuse_en "Computing", reuse_de "Berechnen";
        status_error: native_en "Error", native_de "Fehler", reuse_en "Error", reuse_de "Fehler";
        status_blocked: native_en "Blocked", native_de "Blockiert", reuse_en "Blocked", reuse_de "Blockiert";
        graph_canvas: native_en "Node graph canvas", native_de "Knotengraph-Leinwand", reuse_en "Node graph canvas", reuse_de "Knotengraph-Leinwand";
        graph_canvas_hint: native_en "Interactive node graph. The Artifact panel lists every node, port and wire for keyboard and screen-reader use.", native_de "Interaktiver Knotengraph. Das Artefakt-Panel listet alle Knoten, Anschlüsse und Leitungen für Tastatur und Screenreader.", reuse_en "Interactive node graph. The Artifact panel lists every node, port and wire for keyboard and screen-reader use.", reuse_de "Interaktiver Knotengraph. Das Artefakt-Panel listet alle Knoten, Anschlüsse und Leitungen für Tastatur und Screenreader.";
        preview_canvas: native_en "3D preview canvas", native_de "3D-Vorschau-Leinwand", reuse_en "3D preview canvas", reuse_de "3D-Vorschau-Leinwand";
        preview_canvas_hint: native_en "Interactive 3D preview of the evaluated flow. Drag to orbit, click to select; the evaluation status is announced as it changes.", native_de "Interaktive 3D-Vorschau des ausgewerteten Workflows. Ziehen zum Umkreisen, Klicken zum Auswählen; der Auswertungsstatus wird bei Änderung angesagt.", reuse_en "Interactive 3D preview of the evaluated flow. Drag to orbit, click to select; the evaluation status is announced as it changes.", reuse_de "Interaktive 3D-Vorschau des ausgewerteten Workflows. Ziehen zum Umkreisen, Klicken zum Auswählen; der Auswertungsstatus wird bei Änderung angesagt.";
    }
}

/// 🛍️ Resolves catalogue operator names in the explicitly chosen language.
pub fn generation3d_catalogue_name<'a>(labels: &'a Generation3dLabels, id: &str, name: &'a str) -> &'a str {
    match id {
        "inputs" => labels.catalogue_inputs.as_str(),
        "outputs" => labels.catalogue_outputs.as_str(),
        "inputSlider" => labels.catalogue_input_slider.as_str(),
        "inputNote" => labels.catalogue_input_note.as_str(),
        "inputImage" => labels.catalogue_input_image.as_str(),
        "variable" => labels.catalogue_variable.as_str(),
        "outputPreview" => labels.catalogue_output_preview.as_str(),
        "outputAction" => labels.catalogue_output_action.as_str(),
        "outputExport" => labels.catalogue_output_export.as_str(),
        "brep.mesh.inspectVertex" => labels.catalogue_brep_mesh_inspect_vertex.as_str(),
        "brep.mesh.inspectEdge" => labels.catalogue_brep_mesh_inspect_edge.as_str(),
        "brep.mesh.inspectFace" => labels.catalogue_brep_mesh_inspect_face.as_str(),
        "brep.mesh.bevel" => labels.catalogue_brep_mesh_bevel.as_str(),
        "brep.mesh.dissolveEdges" => labels.catalogue_brep_mesh_dissolve_edges.as_str(),
        "brep.mesh.dissolveVertices" => labels.catalogue_brep_mesh_dissolve_vertices.as_str(),
        "brep.mesh.mergeVertices" => labels.catalogue_brep_mesh_merge_vertices.as_str(),
        "brep.mesh.moveProportional" => labels.catalogue_brep_mesh_move_proportional.as_str(),
        "brep.mesh.snapVertices" => labels.catalogue_brep_mesh_snap_vertices.as_str(),
        "brep.mesh.mirror" => labels.catalogue_brep_mesh_mirror.as_str(),
        "brep.mesh.decimate" => labels.catalogue_brep_mesh_decimate.as_str(),
        "brep.mesh.mergeCoplanar" => labels.catalogue_brep_mesh_merge_coplanar.as_str(),
        "brep.bool.compoundCut" => labels.catalogue_brep_bool_compound_cut.as_str(),
        "brep.bool.cut" => labels.catalogue_brep_bool_cut.as_str(),
        "brep.bool.fuse" => labels.catalogue_brep_bool_fuse.as_str(),
        "brep.bool.intersect" => labels.catalogue_brep_bool_intersect.as_str(),
        "brep.brep" => labels.catalogue_brep_brep.as_str(),
        "brep.curve.approximate" => labels.catalogue_brep_curve_approximate.as_str(),
        "brep.curve.arc" => labels.catalogue_brep_curve_arc.as_str(),
        "brep.curve.circle" => labels.catalogue_brep_curve_circle.as_str(),
        "brep.curve.ellipse" => labels.catalogue_brep_curve_ellipse.as_str(),
        "brep.curve.helix" => labels.catalogue_brep_curve_helix.as_str(),
        "brep.curve.interpolate" => labels.catalogue_brep_curve_interpolate.as_str(),
        "brep.curve.line" => labels.catalogue_brep_curve_line.as_str(),
        "brep.curve.polygon" => labels.catalogue_brep_curve_polygon.as_str(),
        "brep.curve.polyline" => labels.catalogue_brep_curve_polyline.as_str(),
        "brep.curve.rectangle" => labels.catalogue_brep_curve_rectangle.as_str(),
        "brep.edge" => labels.catalogue_brep_edge.as_str(),
        "brep.eval.curveClosestParameter" => labels.catalogue_brep_eval_curve_closest_parameter.as_str(),
        "brep.eval.curveCurvature" => labels.catalogue_brep_eval_curve_curvature.as_str(),
        "brep.eval.curveDomain" => labels.catalogue_brep_eval_curve_domain.as_str(),
        "brep.eval.curvePoint" => labels.catalogue_brep_eval_curve_point.as_str(),
        "brep.eval.curveTangent" => labels.catalogue_brep_eval_curve_tangent.as_str(),
        "brep.eval.surfNormal" => labels.catalogue_brep_eval_surf_normal.as_str(),
        "brep.eval.surfPoint" => labels.catalogue_brep_eval_surf_point.as_str(),
        "brep.eval.surfaceClosestUv" => labels.catalogue_brep_eval_surface_closest_uv.as_str(),
        "brep.face" => labels.catalogue_brep_face.as_str(),
        "brep.geometry" => labels.catalogue_brep_geometry.as_str(),
        "brep.intersect.curveCurve" => labels.catalogue_brep_intersect_curve_curve.as_str(),
        "brep.intersect.curveSurface" => labels.catalogue_brep_intersect_curve_surface.as_str(),
        "brep.intersect.section" => labels.catalogue_brep_intersect_section.as_str(),
        "brep.intersect.split" => labels.catalogue_brep_intersect_split.as_str(),
        "brep.intersect.surfaceSurface" => labels.catalogue_brep_intersect_surface_surface.as_str(),
        "brep.io.exportDwg" => labels.catalogue_brep_io_export_dwg.as_str(),
        "brep.io.exportObj" => labels.catalogue_brep_io_export_obj.as_str(),
        "brep.io.exportStep" => labels.catalogue_brep_io_export_step.as_str(),
        "brep.io.exportStl" => labels.catalogue_brep_io_export_stl.as_str(),
        "brep.io.importDwg" => labels.catalogue_brep_io_import_dwg.as_str(),
        "brep.io.importObj" => labels.catalogue_brep_io_import_obj.as_str(),
        "brep.io.importStep" => labels.catalogue_brep_io_import_step.as_str(),
        "brep.io.importStl" => labels.catalogue_brep_io_import_stl.as_str(),
        "brep.measure.area" => labels.catalogue_brep_measure_area.as_str(),
        "brep.measure.boundingBox" => labels.catalogue_brep_measure_bounding_box.as_str(),
        "brep.measure.centerOfMass" => labels.catalogue_brep_measure_center_of_mass.as_str(),
        "brep.measure.classify" => labels.catalogue_brep_measure_classify.as_str(),
        "brep.measure.closestPoint" => labels.catalogue_brep_measure_closest_point.as_str(),
        "brep.measure.distance" => labels.catalogue_brep_measure_distance.as_str(),
        "brep.measure.length" => labels.catalogue_brep_measure_length.as_str(),
        "brep.measure.validate" => labels.catalogue_brep_measure_validate.as_str(),
        "brep.measure.volume" => labels.catalogue_brep_measure_volume.as_str(),
        "brep.mesh" => labels.catalogue_brep_mesh.as_str(),
        "brep.mesh.analyze" => labels.catalogue_brep_mesh_analyze.as_str(),
        "brep.mesh.box" => labels.catalogue_brep_mesh_box.as_str(),
        "brep.mesh.cone" => labels.catalogue_brep_mesh_cone.as_str(),
        "brep.mesh.construct" => labels.catalogue_brep_mesh_construct.as_str(),
        "brep.mesh.cylinder" => labels.catalogue_brep_mesh_cylinder.as_str(),
        "brep.mesh.deleteFaces" => labels.catalogue_brep_mesh_delete_faces.as_str(),
        "brep.mesh.exportJson" => labels.catalogue_brep_mesh_export_json.as_str(),
        "brep.mesh.exportObj" => labels.catalogue_brep_mesh_export_obj.as_str(),
        "brep.mesh.extrude" => labels.catalogue_brep_mesh_extrude.as_str(),
        "brep.mesh.fillHoles" => labels.catalogue_brep_mesh_fill_holes.as_str(),
        "brep.mesh.flip" => labels.catalogue_brep_mesh_flip.as_str(),
        "brep.mesh.fromBrep" => labels.catalogue_brep_mesh_from_brep.as_str(),
        "brep.mesh.inset" => labels.catalogue_brep_mesh_inset.as_str(),
        "brep.mesh.knifeCut" => labels.catalogue_brep_mesh_knife_cut.as_str(),
        "brep.mesh.loopCut" => labels.catalogue_brep_mesh_loop_cut.as_str(),
        "brep.mesh.moveVertices" => labels.catalogue_brep_mesh_move_vertices.as_str(),
        "brep.mesh.orient" => labels.catalogue_brep_mesh_orient.as_str(),
        "brep.mesh.plane" => labels.catalogue_brep_mesh_plane.as_str(),
        "brep.mesh.rotate" => labels.catalogue_brep_mesh_rotate.as_str(),
        "brep.mesh.rotateComponents" => labels.catalogue_brep_mesh_rotate_components.as_str(),
        "brep.mesh.scale" => labels.catalogue_brep_mesh_scale.as_str(),
        "brep.mesh.scaleComponents" => labels.catalogue_brep_mesh_scale_components.as_str(),
        "brep.mesh.sphere" => labels.catalogue_brep_mesh_sphere.as_str(),
        "brep.mesh.subdivide" => labels.catalogue_brep_mesh_subdivide.as_str(),
        "brep.mesh.toBrep" => labels.catalogue_brep_mesh_to_brep.as_str(),
        "brep.mesh.translate" => labels.catalogue_brep_mesh_translate.as_str(),
        "brep.mesh.translateComponents" => labels.catalogue_brep_mesh_translate_components.as_str(),
        "brep.mesh.triangulate" => labels.catalogue_brep_mesh_triangulate.as_str(),
        "brep.mesh.weld" => labels.catalogue_brep_mesh_weld.as_str(),
        "brep.prim3d.box" => labels.catalogue_brep_prim3d_box.as_str(),
        "brep.prim3d.cone" => labels.catalogue_brep_prim3d_cone.as_str(),
        "brep.prim3d.convexHull" => labels.catalogue_brep_prim3d_convex_hull.as_str(),
        "brep.prim3d.cylinder" => labels.catalogue_brep_prim3d_cylinder.as_str(),
        "brep.prim3d.sphere" => labels.catalogue_brep_prim3d_sphere.as_str(),
        "brep.prim3d.torus" => labels.catalogue_brep_prim3d_torus.as_str(),
        "brep.solid.chamfer" => labels.catalogue_brep_solid_chamfer.as_str(),
        "brep.solid.chamferAsymmetric" => labels.catalogue_brep_solid_chamfer_asymmetric.as_str(),
        "brep.solid.chamferEdges" => labels.catalogue_brep_solid_chamfer_edges.as_str(),
        "brep.solid.defeature" => labels.catalogue_brep_solid_defeature.as_str(),
        "brep.solid.draft" => labels.catalogue_brep_solid_draft.as_str(),
        "brep.solid.extrude" => labels.catalogue_brep_solid_extrude.as_str(),
        "brep.solid.fillet" => labels.catalogue_brep_solid_fillet.as_str(),
        "brep.solid.filletEdges" => labels.catalogue_brep_solid_fillet_edges.as_str(),
        "brep.solid.filletVariable" => labels.catalogue_brep_solid_fillet_variable.as_str(),
        "brep.solid.offsetSolid" => labels.catalogue_brep_solid_offset_solid.as_str(),
        "brep.solid.shell" => labels.catalogue_brep_solid_shell.as_str(),
        "brep.surf.coons" => labels.catalogue_brep_surf_coons.as_str(),
        "brep.surf.nurbsGrid" => labels.catalogue_brep_surf_nurbs_grid.as_str(),
        "brep.surf.offset" => labels.catalogue_brep_surf_offset.as_str(),
        "brep.surf.planarFace" => labels.catalogue_brep_surf_planar_face.as_str(),
        "brep.surf.planarFaceWire" => labels.catalogue_brep_surf_planar_face_wire.as_str(),
        "brep.surf.plane" => labels.catalogue_brep_surf_plane.as_str(),
        "brep.surf.thicken" => labels.catalogue_brep_surf_thicken.as_str(),
        "brep.sweep.extrude" => labels.catalogue_brep_sweep_extrude.as_str(),
        "brep.sweep.helical" => labels.catalogue_brep_sweep_helical.as_str(),
        "brep.sweep.loft" => labels.catalogue_brep_sweep_loft.as_str(),
        "brep.sweep.pipe" => labels.catalogue_brep_sweep_pipe.as_str(),
        "brep.sweep.revolve" => labels.catalogue_brep_sweep_revolve.as_str(),
        "brep.sweep.sweep" => labels.catalogue_brep_sweep_sweep.as_str(),
        "brep.text" => labels.catalogue_brep_text.as_str(),
        "brep.topology.compound" => labels.catalogue_brep_topology_compound.as_str(),
        "brep.topology.explode" => labels.catalogue_brep_topology_explode.as_str(),
        "brep.topology.label" => labels.catalogue_brep_topology_label.as_str(),
        "brep.topology.shells" => labels.catalogue_brep_topology_shells.as_str(),
        "brep.util.convertToNurbs" => labels.catalogue_brep_util_convert_to_nurbs.as_str(),
        "brep.util.faceFromWire" => labels.catalogue_brep_util_face_from_wire.as_str(),
        "brep.util.heal" => labels.catalogue_brep_util_heal.as_str(),
        "brep.util.sew" => labels.catalogue_brep_util_sew.as_str(),
        "brep.util.vertex" => labels.catalogue_brep_util_vertex.as_str(),
        "brep.vertex" => labels.catalogue_brep_vertex.as_str(),
        "brep.xform.circularPattern" => labels.catalogue_brep_xform_circular_pattern.as_str(),
        "brep.xform.copy" => labels.catalogue_brep_xform_copy.as_str(),
        "brep.xform.gridPattern" => labels.catalogue_brep_xform_grid_pattern.as_str(),
        "brep.xform.linearPattern" => labels.catalogue_brep_xform_linear_pattern.as_str(),
        "brep.xform.mirror" => labels.catalogue_brep_xform_mirror.as_str(),
        "brep.xform.rotate" => labels.catalogue_brep_xform_rotate.as_str(),
        "brep.xform.rotateAbout" => labels.catalogue_brep_xform_rotate_about.as_str(),
        "brep.xform.scale" => labels.catalogue_brep_xform_scale.as_str(),
        "brep.xform.translate" => labels.catalogue_brep_xform_translate.as_str(),
        "input" => labels.catalogue_input.as_str(),
        "output" => labels.catalogue_output.as_str(),
        "contract" => labels.catalogue_contract.as_str(),
        "brep" => labels.catalogue_brep.as_str(),
        _ => name,
    }
}

/// 🎛️ Resolves operator input names in the explicitly chosen language.
pub fn generation3d_input_name<'a>(labels: &'a Generation3dLabels, id: &str, name: &'a str) -> &'a str {
    match id {
        "a" => labels.input_name_a.as_str(),
        "b" => labels.input_name_b.as_str(),
        "amount" => labels.input_name_amount.as_str(),
        "angle" => labels.input_name_angle.as_str(),
        "axis" => labels.input_name_axis.as_str(),
        "axisDirection" => labels.input_name_axis_direction.as_str(),
        "axisOrigin" => labels.input_name_axis_origin.as_str(),
        "brep" => labels.input_name_brep.as_str(),
        "center" => labels.input_name_center.as_str(),
        "compound" => labels.input_name_compound.as_str(),
        "controlPoints" => labels.input_name_control_points.as_str(),
        "count" => labels.input_name_count.as_str(),
        "countX" => labels.input_name_count_x.as_str(),
        "countY" => labels.input_name_count_y.as_str(),
        "curve" => labels.input_name_curve.as_str(),
        "curves" => labels.input_name_curves.as_str(),
        "cuts" => labels.input_name_cuts.as_str(),
        "d1" => labels.input_name_d1.as_str(),
        "d2" => labels.input_name_d2.as_str(),
        "data" => labels.input_name_data.as_str(),
        "deflection" => labels.input_name_deflection.as_str(),
        "degree" => labels.input_name_degree.as_str(),
        "degreeU" => labels.input_name_degree_u.as_str(),
        "degreeV" => labels.input_name_degree_v.as_str(),
        "depth" => labels.input_name_depth.as_str(),
        "dirX" => labels.input_name_dir_x.as_str(),
        "dirY" => labels.input_name_dir_y.as_str(),
        "direction" => labels.input_name_direction.as_str(),
        "distance" => labels.input_name_distance.as_str(),
        "edge" => labels.input_name_edge.as_str(),
        "edges" => labels.input_name_edges.as_str(),
        "end" => labels.input_name_end.as_str(),
        "endAngle" => labels.input_name_end_angle.as_str(),
        "face" => labels.input_name_face.as_str(),
        "faces" => labels.input_name_faces.as_str(),
        "factor" => labels.input_name_factor.as_str(),
        "geometry" => labels.input_name_geometry.as_str(),
        "guide" => labels.input_name_guide.as_str(),
        "handle" => labels.input_name_handle.as_str(),
        "height" => labels.input_name_height.as_str(),
        "index" => labels.input_name_index.as_str(),
        "kind" => labels.input_name_kind.as_str(),
        "major" => labels.input_name_major.as_str(),
        "mesh" => labels.input_name_mesh.as_str(),
        "minor" => labels.input_name_minor.as_str(),
        "mode" => labels.input_name_mode.as_str(),
        "neutralPoint" => labels.input_name_neutral_point.as_str(),
        "normal" => labels.input_name_normal.as_str(),
        "offset" => labels.input_name_offset.as_str(),
        "openFaces" => labels.input_name_open_faces.as_str(),
        "origin" => labels.input_name_origin.as_str(),
        "parameter" => labels.input_name_parameter.as_str(),
        "path" => labels.input_name_path.as_str(),
        "pitch" => labels.input_name_pitch.as_str(),
        "pivot" => labels.input_name_pivot.as_str(),
        "planeNormal" => labels.input_name_plane_normal.as_str(),
        "planeOrigin" => labels.input_name_plane_origin.as_str(),
        "point" => labels.input_name_point.as_str(),
        "points" => labels.input_name_points.as_str(),
        "preview" => labels.input_name_preview.as_str(),
        "profile" => labels.input_name_profile.as_str(),
        "profiles" => labels.input_name_profiles.as_str(),
        "pullDirection" => labels.input_name_pull_direction.as_str(),
        "radius" => labels.input_name_radius.as_str(),
        "radiusEnd" => labels.input_name_radius_end.as_str(),
        "radiusStart" => labels.input_name_radius_start.as_str(),
        "rows" => labels.input_name_rows.as_str(),
        "segments" => labels.input_name_segments.as_str(),
        "selection" => labels.input_name_selection.as_str(),
        "semiMajor" => labels.input_name_semi_major.as_str(),
        "semiMinor" => labels.input_name_semi_minor.as_str(),
        "sides" => labels.input_name_sides.as_str(),
        "smooth" => labels.input_name_smooth.as_str(),
        "solid" => labels.input_name_solid.as_str(),
        "solids" => labels.input_name_solids.as_str(),
        "spacing" => labels.input_name_spacing.as_str(),
        "spacingX" => labels.input_name_spacing_x.as_str(),
        "spacingY" => labels.input_name_spacing_y.as_str(),
        "start" => labels.input_name_start.as_str(),
        "startAngle" => labels.input_name_start_angle.as_str(),
        "subdivisions" => labels.input_name_subdivisions.as_str(),
        "surface" => labels.input_name_surface.as_str(),
        "target" => labels.input_name_target.as_str(),
        "text" => labels.input_name_text.as_str(),
        "thickness" => labels.input_name_thickness.as_str(),
        "tolerance" => labels.input_name_tolerance.as_str(),
        "tools" => labels.input_name_tools.as_str(),
        "turns" => labels.input_name_turns.as_str(),
        "u" => labels.input_name_u.as_str(),
        "v" => labels.input_name_v.as_str(),
        "value" => labels.input_name_value.as_str(),
        "vector" => labels.input_name_vector.as_str(),
        "vertex" => labels.input_name_vertex.as_str(),
        "vertices" => labels.input_name_vertices.as_str(),
        "width" => labels.input_name_width.as_str(),
        "wire" => labels.input_name_wire.as_str(),
        "grid" => labels.input_name_grid.as_str(),
        "ratio" => labels.input_name_ratio.as_str(),
        "components" => labels.input_name_components.as_str(),
        "granularity" => labels.input_name_granularity.as_str(),
        "x" => labels.input_name_x.as_str(),
        "y" => labels.input_name_y.as_str(),
        "z" => labels.input_name_z.as_str(),
        "number" => labels.input_name_number.as_str(),
        "numbers" => labels.input_name_numbers.as_str(),
        "axisName" => labels.input_name_axis_name.as_str(),
        _ => name,
    }
}

/// 🗣️ Resolves the active label set from the shared view model.
pub fn generation3d_labels(view_state: &semio_framework_plugin::ViewModel) -> &'static Generation3dLabels {
    semio_framework_plugin::resolve_labels::<Generation3dLabels>(view_state)
}

//#endregion 🔖️Labels

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
