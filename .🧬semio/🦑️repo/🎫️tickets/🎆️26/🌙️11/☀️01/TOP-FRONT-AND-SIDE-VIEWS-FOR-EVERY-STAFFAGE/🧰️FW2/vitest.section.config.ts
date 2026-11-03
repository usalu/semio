import base from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🎚️config/🟦️.ts";

export default {
  ...base,
  test: {
    ...base.test,
    include: ["../../🧪️tests/✂️section-views/🟦️.ts", "../../🧪️tests/📐️dimension-clearance/🟦️.ts", "../../🧪️tests/🔤️caption-fit/🟦️.ts"],
  },
};
