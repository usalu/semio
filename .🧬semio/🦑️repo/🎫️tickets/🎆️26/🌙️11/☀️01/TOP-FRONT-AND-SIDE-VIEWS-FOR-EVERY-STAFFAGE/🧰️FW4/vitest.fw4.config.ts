import base from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🎚️config/🟦️.ts";

export default {
  ...base,
  test: {
    ...base.test,
    include: ["../../🧪️tests/🚀️export-performance/🟦️.ts", "../../🧪️tests/📏️print-size/🟦️.ts", "../../🧪️tests/🔤️caption-fit/🟦️.ts"],
  },
};
