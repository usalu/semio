import base from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🎚️config/🟦️.ts";

export default {
  ...base,
  test: {
    ...base.test,
    include: ["../../🧪️tests/🧍️front-projection/🟦️.ts", "../../🧪️tests/🧊️three-views/🟦️.ts"],
  },
};
