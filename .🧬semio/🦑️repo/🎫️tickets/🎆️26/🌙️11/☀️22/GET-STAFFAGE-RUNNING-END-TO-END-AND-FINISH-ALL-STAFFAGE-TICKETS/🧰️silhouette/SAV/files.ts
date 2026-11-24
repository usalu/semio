/** 🗺️ The 23 assets of batch SAV and their catalog files. */
export const R = "/Users/ueli/Documents/semio";
export const S = `${R}/🎓️teaching/🏛️architecture/🧍️staffage`;
export const ANIMALS = ["dairy-cow", "domestic-cat", "goat", "hedgehog", "herring-gull", "horse", "labrador-dog", "mallard-duck", "pig", "pigeon", "red-fox", "red-squirrel", "roe-deer", "sheep"];
export const VEHICLES = ["cargo-bike", "city-bicycle", "city-bus", "city-car", "delivery-van", "electric-kick-scooter", "motor-scooter", "tram", "vintage-car-1929"];
export const catalogFile = (id: string): string => {
  if (ANIMALS.includes(id)) return `${S}/🗂️catalog/🐕️animals/${id}/🔣️.json`;
  if (VEHICLES.includes(id)) return `${S}/🗂️catalog/🚜️vehicles/${id}/🔣️.json`;
  throw new Error(`${id} is not an asset of batch SAV`);
};
export const fileOf = (arg: string): string => (arg.endsWith(".json") ? arg : catalogFile(arg));
