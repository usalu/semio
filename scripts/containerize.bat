cd src\backend\gateway\server
pack build semiogateway:base --builder=paketobuildpacks/builder:base
cd ..
cd restproxy
pack build semiogatewayrestproxy:tiny --builder=paketobuildpacks/builder:tiny
cd ..
cd ..
cd manager
pack build semiomanager:base --builder=paketobuildpacks/builder:base
cd ..
cd assembler
pack build semiomanager:base --builder=paketobuildpacks/builder:base
cd ..
cd ..
cd extensions\grasshopper
pack build semioextensiongrasshopper:base --builder=paketobuildpacks/builder:base
cd ..
cd ..
cd ..