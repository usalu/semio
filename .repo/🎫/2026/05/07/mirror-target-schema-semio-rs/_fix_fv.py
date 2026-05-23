from pathlib import Path

p = Path("semio/rs/gql_target.rs")
t = p.read_text(encoding="utf-8")
reps = [
    ("Ok(Some(FieldValue::from(h.finalize().to_hex().to_string())))", "Ok(Some(fv_str(h.finalize().to_hex().to_string())))"),
    ("Ok(Some(FieldValue::from(p.cursor.clone())))", "Ok(Some(fv_str(p.cursor.clone())))"),
    (
        "FieldFuture::new(async move { Ok(Some(FieldValue::from(false))) })",
        "FieldFuture::new(async move { Ok(Some(fv_bool(false))) })",
    ),
    (
        "FieldFuture::new(async move { Ok(Some(FieldValue::from(String::new()))) })",
        'FieldFuture::new(async move { Ok(Some(fv_str(""))) })',
    ),
    ("Ok(Some(FieldValue::from(g.id.clone())))", "Ok(Some(fv_str(g.id.as_str())))"),
    ("Ok(Some(FieldValue::from(g.compute_hash().await)))", "Ok(Some(fv_str(g.compute_hash().await)))"),
    ("Ok(Some(FieldValue::from(k.id.clone())))", "Ok(Some(fv_str(k.id.as_str())))"),
    ("Ok(Some(FieldValue::from(k.compute_hash().await)))", "Ok(Some(fv_str(k.compute_hash().await)))"),
    ("Ok(Some(FieldValue::from(k.name.read().await.clone())))", "Ok(Some(fv_str(k.name.read().await.clone())))"),
    ("Ok(Some(FieldValue::from(d.id.clone())))", "Ok(Some(fv_str(d.id.as_str())))"),
    ("Ok(Some(FieldValue::from(d.compute_hash().await)))", "Ok(Some(fv_str(d.compute_hash().await)))"),
    ("Ok(Some(FieldValue::from(d.name.read().await.clone())))", "Ok(Some(fv_str(d.name.read().await.clone())))"),
    (
        "FieldFuture::new(async move { Ok(Some(FieldValue::from(0.0f64))) })",
        "FieldFuture::new(async move { Ok(Some(fv_f64(0.0)) })",
    ),
    ("Ok(Some(FieldValue::from(p.id.clone())))", "Ok(Some(fv_str(p.id.as_str())))"),
    ("Ok(Some(FieldValue::from(p.compute_hash().await)))", "Ok(Some(fv_str(p.compute_hash().await)))"),
    (
        "FieldFuture::new(async move { Ok(Some(FieldValue::from(0i32))) })",
        "FieldFuture::new(async move { Ok(Some(fv_i32(0)) })",
    ),
    ("Ok(Some(FieldValue::from(s.id.clone())))", "Ok(Some(fv_str(s.id.as_str())))"),
    ("Ok(Some(FieldValue::from(c.id.clone())))", "Ok(Some(fv_str(c.id.as_str())))"),
    ("Ok(Some(FieldValue::from(c.reason.read().await.clone())))", "Ok(Some(fv_str(c.reason.read().await.clone())))"),
    (
        "Ok(Some(FieldValue::from(c.created_at.read().await.clone())))",
        "Ok(Some(fv_str(c.created_at.read().await.0.clone())))",
    ),
    ('Ok(Some(FieldValue::from("position".to_string())))', 'Ok(Some(fv_str("position")))'),
    ('Ok(Some(FieldValue::from("coord".to_string())))', 'Ok(Some(fv_str("coord")))'),
    ('Ok(Some(FieldValue::from("plane".to_string())))', 'Ok(Some(fv_str("plane")))'),
    ('Ok(Some(FieldValue::from("pt".to_string())))', 'Ok(Some(fv_str("pt")))'),
    ('Ok(Some(FieldValue::from("vec".to_string())))', 'Ok(Some(fv_str("vec")))'),
    ("Ok(Some(FieldValue::from(c.u)))", "Ok(Some(fv_f64(c.u)))"),
    ("Ok(Some(FieldValue::from(c.v)))", "Ok(Some(fv_f64(c.v)))"),
    ("Ok(Some(FieldValue::from(p.x)))", "Ok(Some(fv_f64(p.x)))"),
    ("Ok(Some(FieldValue::from(p.y)))", "Ok(Some(fv_f64(p.y)))"),
    ("Ok(Some(FieldValue::from(p.z)))", "Ok(Some(fv_f64(p.z)))"),
    ("Ok(Some(FieldValue::from(v.x)))", "Ok(Some(fv_f64(v.x)))"),
    ("Ok(Some(FieldValue::from(v.y)))", "Ok(Some(fv_f64(v.y)))"),
    ("Ok(Some(FieldValue::from(v.z)))", "Ok(Some(fv_f64(v.z)))"),
    ("return Ok(Some(FieldValue::from(String::new())));", 'return Ok(Some(fv_str("")));'),
    ("Ok(Some(FieldValue::from(request_id.to_string())))", "Ok(Some(fv_str(request_id.as_str())))"),
    (".map(FieldValue::from)", ".map(fv_str)"),
]
for a, b in reps:
    c = t.count(a)
    if c == 0:
        print("missing:", a[:70])
    t = t.replace(a, b)
p.write_text(t, encoding="utf-8")
print("ok")
