import { fileURLToPath } from "node:url";
import configureCourse from "../../../../../../../🎓️teaching/🏛️architecture/📚️course/🏗️builder/🌐️vite/🟦️.ts";

/** 🌐️ Runs the production course UI against its local deputy for phone browser checks without print artifacts. */
export default (environment: Parameters<typeof configureCourse>[0]) => {
  process.env.SEMIO_VITE_CACHE_DIRECTORY = fileURLToPath(new URL("../../../../../../../.cache/b/p/m/phone-course", import.meta.url));
  process.env.TEACHING_ARCHITECTURE_COURSE_WATCH = "off";
  const course = configureCourse(environment);
  return {
    ...course,
    plugins: course.plugins?.filter((plugin) => plugin.name !== "architecture-course-assignment-pdfs"),
    optimizeDeps: { ...course.optimizeDeps, include: ["react", "react-dom", "react-dom/client", "react/jsx-runtime", "react/jsx-dev-runtime"] },
    server: { ...course.server, proxy: {}, warmup: { clientFiles: ["./🟦️.ts"] } },
  };
};
