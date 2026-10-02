CREATE TABLE shooting_document (id INTEGER PRIMARY KEY CHECK(id=1),schema TEXT NOT NULL,active_shot_id TEXT NOT NULL,active_asset_id TEXT NOT NULL);
CREATE TABLE shooting_scene (
 id INTEGER PRIMARY KEY CHECK(id=1),document_id INTEGER NOT NULL REFERENCES shooting_document(id) CHECK(document_id=1),
 background TEXT NOT NULL,sun_enabled INTEGER NOT NULL CHECK(sun_enabled IN (0,1)),sun_azimuth REAL,sun_elevation REAL,sun_intensity REAL,sun_color TEXT NOT NULL,
 ambient_intensity REAL,ambient_color TEXT NOT NULL,shadow_enabled INTEGER NOT NULL CHECK(shadow_enabled IN (0,1)),shadow_opacity REAL,shadow_softness REAL,
 material_color TEXT NOT NULL,material_metalness REAL,material_roughness REAL,material_emissive TEXT NOT NULL,material_emissive_intensity REAL,material_stroke TEXT NOT NULL,
 sun_azimuth_bits INTEGER NOT NULL,sun_azimuth_class TEXT NOT NULL CHECK(sun_azimuth_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 sun_elevation_bits INTEGER NOT NULL,sun_elevation_class TEXT NOT NULL CHECK(sun_elevation_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 sun_intensity_bits INTEGER NOT NULL,sun_intensity_class TEXT NOT NULL CHECK(sun_intensity_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 ambient_intensity_bits INTEGER NOT NULL,ambient_intensity_class TEXT NOT NULL CHECK(ambient_intensity_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 shadow_opacity_bits INTEGER NOT NULL,shadow_opacity_class TEXT NOT NULL CHECK(shadow_opacity_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 shadow_softness_bits INTEGER NOT NULL,shadow_softness_class TEXT NOT NULL CHECK(shadow_softness_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 material_metalness_bits INTEGER NOT NULL,material_metalness_class TEXT NOT NULL CHECK(material_metalness_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 material_roughness_bits INTEGER NOT NULL,material_roughness_class TEXT NOT NULL CHECK(material_roughness_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 material_emissive_intensity_bits INTEGER NOT NULL,material_emissive_intensity_class TEXT NOT NULL CHECK(material_emissive_intensity_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);
CREATE TABLE shooting_asset (
 id INTEGER PRIMARY KEY CHECK(id>0),document_id INTEGER NOT NULL REFERENCES shooting_document(id) CHECK(document_id=1),ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 asset_id TEXT NOT NULL,name TEXT NOT NULL,url TEXT NOT NULL,format TEXT NOT NULL,origin_x REAL,origin_y REAL,origin_z REAL,
 orientation_x REAL,orientation_y REAL,orientation_z REAL,orientation_w REAL,scale_x REAL,scale_y REAL,scale_z REAL,
 origin_x_bits INTEGER NOT NULL,origin_x_class TEXT NOT NULL CHECK(origin_x_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 origin_y_bits INTEGER NOT NULL,origin_y_class TEXT NOT NULL CHECK(origin_y_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 origin_z_bits INTEGER NOT NULL,origin_z_class TEXT NOT NULL CHECK(origin_z_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 orientation_x_bits INTEGER,orientation_x_class TEXT CHECK(orientation_x_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 orientation_y_bits INTEGER,orientation_y_class TEXT CHECK(orientation_y_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 orientation_z_bits INTEGER,orientation_z_class TEXT CHECK(orientation_z_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 orientation_w_bits INTEGER,orientation_w_class TEXT CHECK(orientation_w_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 scale_x_bits INTEGER,scale_x_class TEXT CHECK(scale_x_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 scale_y_bits INTEGER,scale_y_class TEXT CHECK(scale_y_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 scale_z_bits INTEGER,scale_z_class TEXT CHECK(scale_z_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);
CREATE TABLE shooting_saved_camera (
 id INTEGER PRIMARY KEY CHECK(id>0),document_id INTEGER NOT NULL REFERENCES shooting_document(id) CHECK(document_id=1),ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 camera_id TEXT NOT NULL,label TEXT NOT NULL,position_x REAL,position_y REAL,position_z REAL,target_x REAL,target_y REAL,target_z REAL,zoom REAL,fov REAL,
 up_x REAL,up_y REAL,up_z REAL,projection TEXT,
 position_x_bits INTEGER NOT NULL,position_x_class TEXT NOT NULL CHECK(position_x_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 position_y_bits INTEGER NOT NULL,position_y_class TEXT NOT NULL CHECK(position_y_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 position_z_bits INTEGER NOT NULL,position_z_class TEXT NOT NULL CHECK(position_z_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 target_x_bits INTEGER NOT NULL,target_x_class TEXT NOT NULL CHECK(target_x_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 target_y_bits INTEGER NOT NULL,target_y_class TEXT NOT NULL CHECK(target_y_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 target_z_bits INTEGER NOT NULL,target_z_class TEXT NOT NULL CHECK(target_z_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 zoom_bits INTEGER NOT NULL,zoom_class TEXT NOT NULL CHECK(zoom_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 fov_bits INTEGER NOT NULL,fov_class TEXT NOT NULL CHECK(fov_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 up_x_bits INTEGER,up_x_class TEXT CHECK(up_x_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 up_y_bits INTEGER,up_y_class TEXT CHECK(up_y_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 up_z_bits INTEGER,up_z_class TEXT CHECK(up_z_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);
CREATE TABLE shooting_shot (
 id INTEGER PRIMARY KEY CHECK(id>0),document_id INTEGER NOT NULL REFERENCES shooting_document(id) CHECK(document_id=1),ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 shot_id TEXT NOT NULL,label TEXT NOT NULL,width INTEGER NOT NULL CHECK(width BETWEEN 0 AND 4294967295),height INTEGER NOT NULL CHECK(height BETWEEN 0 AND 4294967295),
 format TEXT NOT NULL,shape TEXT NOT NULL,background TEXT,camera_id TEXT
);
CREATE TABLE shooting_emblem (
 id INTEGER PRIMARY KEY CHECK(id>0),document_id INTEGER NOT NULL REFERENCES shooting_document(id) CHECK(document_id=1),
 child_id TEXT NOT NULL,artifact_id TEXT NOT NULL,artifact_kind TEXT NOT NULL,standard TEXT NOT NULL,subset TEXT NOT NULL
);
