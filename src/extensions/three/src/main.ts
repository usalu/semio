import {Server,ServerUnaryCall,sendUnaryData,ServerCredentials,ChannelCredentials} from '@grpc/grpc-js';
import {GrpcTransport} from "@protobuf-ts/grpc-transport";
import {Representation} from 'semio/model/v1/model'
import {RepresentationConversionRequest} from 'semio/extension/converter/v1/converter'
import {IConverterService,converterServiceDefinition} from 'semio/extension/converter/v1/converter.grpc-server'
import {ServiceRegistrationRequest} from 'semio/server/v1/server'
import {ServerServiceClient} from 'semio/server/v1/server.client'
import { error } from 'console';

const name = "semio.three.js"
const host = '[::]:5000';

const threeConverterService: IConverterService = {

    convertRepresentation(call: ServerUnaryCall<RepresentationConversionRequest, Representation>, callback: sendUnaryData<Representation>): void {
      console.log("Converting representation...")
      callback(null,{name:"HelloRep", lod:BigInt(435)});
    }
}


function getServer(): Server {
    const server = new Server();
    server.addService(converterServiceDefinition, threeConverterService);
    return server;
}

// function convertRepresentation(call : grpc.ServerUnaryCall<RepresentationConversionRequest, Representation>,callback: grpc.sendUnaryData<Representation>){
//     console.log("Converting representation...")
//     callback(null,{name:"HelloRep", lod:BigInt(435)})
// }

// function getServer(): grpc.Server {
//     const server = new grpc.Server();
//     server.addService(converterServiceDefinition, {
//         "convertRepresentation": convertRepresentation
//     });
//     return server;
// }

// function delay(ms: number) {
//     return new Promise( resolve => setTimeout(resolve, ms) );
// }

async function registerExtension() {
    const client = new ServerServiceClient(new GrpcTransport({ host: "localhost:50000", channelCredentials: ChannelCredentials.createInsecure()}))
    const registration = client.registerService({
        replaceExisting: true, 
        serverService: {
            oneofKind: "extendingService",
            extendingService:{
                name:name,
                address:host,
                convertingServices:[
                    {
                        sourceTypeUrl:"mcneel/rhino",
                        targetTypeUrl:"mrdoob/three"
                    }
                ],
                adaptingServices: [],
                transformingServices: []
            }}})
    const response = await registration.response
    // if (response.success===false){
    //     console.log("Attempt to register extension failed. Will try again in 2s.")
    //     await delay(4000).then(() => { registerExtension()})
    // }
    console.log(response)
}


if (require.main === module) {
    const server = getServer();
    server.bindAsync(
        host,
        ServerCredentials.createInsecure(),
        (err: Error | null, port: number) => {
            if (err) {
                console.error(`Server error: ${err.message}`);
            } else {
                console.log(`Server bound on port: ${port}`);
                registerExtension()
                server.start();
            }
        }
    );
}

