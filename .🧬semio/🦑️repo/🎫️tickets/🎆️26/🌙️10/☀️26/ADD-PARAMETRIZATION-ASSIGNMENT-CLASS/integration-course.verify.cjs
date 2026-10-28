const fs=require("fs"),assert=require("node:assert/strict"),Ajv=require("ajv/dist/2020").default;
const base=process.argv[2],fixture=JSON.parse(fs.readFileSync(base+"/🧫️cases.json","utf8")),schema=JSON.parse(fs.readFileSync(base+"/🧬️cases.schema.json","utf8"));
const validate=new Ajv({allErrors:true}).compile(schema);assert.ok(validate(fixture),JSON.stringify(validate.errors));
for(const name of fixture.ownership.files)assert.ok(fs.existsSync(fixture.ownership.sourceRoot+"/"+name),"Course-owned assignment source required: "+name);
const course=JSON.parse(fs.readFileSync("🎓️teaching/🏛️architecture/📚️course/🔣️.json","utf8")),assignment=course.assignments.find(row=>row.id===fixture.ownership.assignment);
assert.equal(assignment.topic,fixture.ownership.topic);assert.deepEqual(assignment.documents,fixture.ownership.documents);
console.log("PASS canonical course assignment ownership and AJV schema");
