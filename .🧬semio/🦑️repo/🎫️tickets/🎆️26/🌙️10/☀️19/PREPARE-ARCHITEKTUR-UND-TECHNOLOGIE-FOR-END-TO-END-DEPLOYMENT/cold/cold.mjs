import value from "@cold/one";
if (process.env.SEMIO_COLD_SELECTED_WORKSPACE !== "authored-fixture") throw Error("Fixture workspace registry was not selected");
console.log(value);
