# PDF1.7 Native IEEE Field Inventory

Research-only inventory of authored SQL scalar positions; numeric domains require explicit native binary64 companions. Existing PDF decimal COS values use exact decimal fields separately.

| Table | Fields (logical column) |
|---|---|
| pdf_function | exponent (7) |
| pdf_function_real | value (4) |
| pdf_color_real | value (4) |
| pdf_font_descriptor | bbox_llx (3), bbox_lly (4), bbox_urx (5), bbox_ury (6), italic_angle (7), ascent (8), descent (9), cap_height (10), stem_v (11), stem_h (12), x_height (13), leading (14), avg_width (15), max_width (16), missing_width (17), font_weight (20) |
| pdf_cid_font | default_width (7), default_vertical_y (8), default_vertical_width (9) |
| pdf_cid_width | width (3) |
| pdf_cid_vertical_metric | w1y (3), vx (4), vy (5) |
| pdf_font | matrix_a (8), matrix_b (9), matrix_c (10), matrix_d (11), matrix_e (12), matrix_f (13), bbox_llx (14), bbox_lly (15), bbox_urx (16), bbox_ury (17) |
| pdf_font_width | width (3) |
| pdf_operation | line_width (4), miter_limit (7), dash_phase (8), flatness (10), x1 (12), y1 (13), x2 (14), y2 (15), x3 (16), y3 (17), width (18), height (19), char_spacing (20), word_spacing (21), horizontal_scale (22), leading (23), font_size (25), text_rise (27), tx (28), ty (29), glyph_wx (33), glyph_wy (34), bbox_llx (35), bbox_lly (36), bbox_urx (37), bbox_ury (38), gray (41), red (42), green (43), blue (44), cyan (45), magenta (46), yellow (47), black (48) |
| pdf_operation_matrix | a (1), b (2), c (3), d (4), e (5), f (6) |
| pdf_operation_component | value (3) |
| pdf_text_array_item | adjustment (6) |
| pdf_inline_decode | value (3) |
| pdf_image_real | value (4) |
| pdf_form_xobject | bbox_llx (2), bbox_lly (3), bbox_urx (4), bbox_ury (5), matrix_a (6), matrix_b (7), matrix_c (8), matrix_d (9), matrix_e (10), matrix_f (11) |
| pdf_ext_g_state | line_width (2), miter_limit (5), dash_phase (6), font_size (12), stroke_alpha (18), fill_alpha (19), flatness (22), smoothness (23) |
| pdf_g_state_real | value (4) |
| pdf_shading | bbox_llx (5), bbox_lly (6), bbox_urx (7), bbox_ury (8) |
| pdf_shading_background | value (3) |
| pdf_function_shading | domain_xmin (1), domain_xmax (2), domain_ymin (3), domain_ymax (4), matrix_a (5), matrix_b (6), matrix_c (7), matrix_d (8), matrix_e (9), matrix_f (10) |
| pdf_axial_shading | x0 (1), y0 (2), x1 (3), y1 (4), domain_start (5), domain_end (6) |
| pdf_radial_shading | x0 (1), y0 (2), r0 (3), x1 (4), y1 (5), r1 (6), domain_start (7), domain_end (8) |
| pdf_mesh_decode | value (3) |
| pdf_pattern | matrix_a (3), matrix_b (4), matrix_c (5), matrix_d (6), matrix_e (7), matrix_f (8) |
| pdf_tiling_pattern | bbox_llx (3), bbox_lly (4), bbox_urx (5), bbox_ury (6), x_step (7), y_step (8) |
| pdf_destination | left (5), top (6), zoom (7), rect_llx (8), rect_lly (9), rect_urx (10), rect_ury (11) |
| pdf_action | volume (9) |
| pdf_outline | red (4), green (5), blue (6) |
| pdf_annotation_border | width (1), radius_x (4), radius_y (5) |
| pdf_border_dash | value (3) |
| pdf_annotation_markup | opacity (4) |
| pdf_annotation | rect_left (1), rect_bottom (2), rect_right (3), rect_top (4) |
| pdf_annotation_real | value (3) |
| pdf_annotation_detail | leader_length (13), point_0 (37), point_1 (38), point_2 (39), point_3 (40) |
| pdf_annotation_kind_real | value (4) |
| pdf_annotation_ink_coordinate | value (3) |
| pdf_page | media_left (1), media_bottom (2), media_right (3), media_top (4), crop_left (5), crop_bottom (6), crop_right (7), crop_top (8), bleed_left (9), bleed_bottom (10), bleed_right (11), bleed_top (12), trim_left (13), trim_bottom (14), trim_right (15), trim_top (16), art_left (17), art_bottom (18), art_right (19), art_top (20), user_unit (22), duration (28) |
